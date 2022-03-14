// TODO(xla): Set up logging/tracing.
// TODO(xla): Set up telemetry (Prometheus): process, futures
// TODO(xla): Extend telemetry with
//              * fetch durations (pools, accounts)
//              * current account balances

use std::{fs, sync::Arc};

use anyhow::Result;
use futures::{FutureExt as _, StreamExt};
use tendermint::{
    abci::Transaction,
    block::{self, Height},
};
use tendermint_rpc::{
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use tokio::sync::{mpsc, watch, Mutex};

use common::{
    metrics,
    tracing::{self, debug, error, info},
};
use terrars::{
    dex,
    lcd::{self, Lcd},
    mekapi, BaseAccount, Message, Msg as _,
};

use execution::{
    accounts,
    block_emitter::{self, Emitter as _},
    config,
    risk_engine::{
        EmptyPoolDeny, RiskClassifier, RiskEngine, StaticPoolDenyList, UnsupportedPoolList,
    },
    simulation::{self, Simulator as _},
    strategy::{self, fetcher::Fetcher as _, Trade},
    tracker::{self, Tracker as _},
};

const BINARY_NAMESPACE: &str = "execution";

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let args: config::Args = argh::from_env();

    let secret_key = {
        let raw = fs::read(args.secret_key_file.clone())?;
        let bytes = hex::decode(&raw)?;
        // FIXME(xla): Proper error handling.
        terrars::SigningKey::from_bytes(&bytes).unwrap()
    };
    // FIXME(xla): Proper error handling.
    let account_id = secret_key
        .public_key()
        .account_id(accounts::PREFIX)
        .unwrap();
    // Set up RPC client.
    let (client, driver) = WebSocketClient::new(args.rpc_url.as_str()).await?;
    // Advance the internal machinery of the RPC client.
    let mut driver_handle = tokio::spawn(driver.run()).fuse();

    // Ensure the data source has caught up to the latest state.
    let status = client.status().await?;
    if status.sync_info.catching_up {
        error!("Node status: {:?}", status);
        return Err(anyhow::anyhow!("node still catching up"));
    }

    // Keep the latest block up-to-date so it can function as barrier for the next iteration of the
    // execution loop. By fetching it first we ensure the watch channel has an initial value.
    let (latest_block_sender, mut latest_block_receiver) = {
        let res = client.latest_block().await?;
        watch::channel(res.block)
    };
    let (trade_sender, mut trade_receiver) = mpsc::channel::<(block::Height, Trade)>(100);

    let mut registry = metrics::Registry::new(BINARY_NAMESPACE);

    let emitter =
        block_emitter::Observed::new(&mut registry, block_emitter::Rpc::new(latest_block_sender));
    let lcd = execution::metrics::lcd::Observed::new(&mut registry, lcd::Http::new(args.lcd_url)?);
    let mekapi = execution::metrics::mekapi::Observed::new(
        &mut registry,
        mekapi::Http::new(args.mekapi_url)?,
    );
    let pool_fetcher =
        strategy::fetcher::Observed::new(&mut registry, strategy::fetcher::Mekapi {});
    let simulator = simulation::Observed::new(&mut registry, simulation::Rpc::new(lcd.clone()));
    // TODO(xla): Load configured strategies.
    let mut strategies: Vec<Box<dyn strategy::Strategy + Sync>> =
        vec![Box::new(strategy::metrics::Observed::new(
            &mut registry,
            strategy::multi_dex::multi_hop::MultiHop::default(),
        ))];
    let tx_tracker = tracker::metrics::Observed::new(&mut registry, tracker::InMemory::default());

    let metrics_server = metrics::bind(args.metrics_addr, BINARY_NAMESPACE, registry);
    let mut metrics_server_handle = tokio::spawn(metrics_server).fuse();

    // Subroutine to keep lastest block up-to-date.
    let mut latest_block_handle = {
        let client = client.clone();
        tokio::spawn(async move {
            debug!("Init block subscription");

            let query = Query::from(EventType::NewBlock);
            let mut events = client.subscribe(query).await?;

            while let Some(res) = events.next().await {
                if let Err(err) = emitter.emit(res) {
                    error!("failed to emit latest block: {}", err);
                }
            }

            Ok::<(), anyhow::Error>(())
        })
        .fuse()
    };

    let tx_tracker = Arc::new(Mutex::new(tx_tracker));
    // Subroutine which compiles, signs and broadcasts message bundles into transactions.
    let mut broadcast_handle = {
        let account_id = account_id.clone();
        let client = client.clone();
        let lcd = lcd.clone();
        let tx_tracker = Arc::clone(&tx_tracker);

        tokio::spawn(async move {
            debug!("Init broadcaster");

            let secret_key_bytes = {
                let raw = fs::read(args.secret_key_file.clone())?;
                hex::decode(&raw)?
            };

            // Get latest account information to ensure sequence is consistent.
            let account = lcd.auth_account(&account_id).await?.unwrap_or(BaseAccount {
                account_number: 0,
                address: account_id.to_string(),
                pub_key: None,
                sequence: 0,
            });
            let mut sequence = account.sequence;

            while let Some((_height, trade)) = trade_receiver.recv().await {
                let route = trade.route;
                if let Some(addr) = (*tx_tracker.lock().await).is_tracked(&route) {
                    debug!("pool {} in route  ", addr);
                    continue;
                }

                let msgs: Vec<Message> = trade
                    .messages
                    .into_iter()
                    .map(|msg| msg.to_any().unwrap())
                    .collect();

                let expected_profit = (trade.expected_profit.0 / 10u64.pow(12).into()).as_u64();

                if let Ok((fee, gas_info)) = simulator
                    .simulate(
                        &secret_key_bytes,
                        args.chain_id.clone(),
                        account.account_number,
                        sequence,
                        msgs.clone(),
                        expected_profit,
                    )
                    .await
                {
                    if !args.dry_run && expected_profit > fee {
                        let tx = terrars::tx::sign(
                            args.chain_id.clone(),
                            // FIXME(xla): Proper error handling.
                            &terrars::SigningKey::from_bytes(&secret_key_bytes).unwrap(),
                            account.account_number,
                            sequence,
                            // TODO(xla): Use actual gas prices.
                            // XXX(xla): Based on the observation that txs with two swaps cost
                            // around 180k and the typical gas price in
                            // uusd is 0.15.
                            fee,
                            (gas_info.gas_used as f64 * 1.10) as u64,
                            Height::try_from(0_u64).unwrap(),
                            msgs,
                        )?;
                        match client.broadcast_tx_sync(Transaction::from(tx)).await {
                            Ok(check) => {
                                if check.code.is_ok() {
                                    sequence += 1;

                                    let _ = (*tx_tracker.lock().await).track(check.hash, route);
                                }
                                info!(
                                    "msg=submit hash={} code={:?} log={}",
                                    check.hash, check.code, check.log
                                );
                            },
                            Err(err) => error!("tx submit failed: {}", err),
                        }
                    }
                }
            }

            Ok::<(), anyhow::Error>(())
        })
        .fuse()
    };

    // Subroutine to keep the dependent information for strategies up-to-date and execute
    // strategies.
    let mut execution_handle = tokio::spawn(async move {
        debug!("Init strategy executor");

        let risk: RiskEngine<dyn RiskClassifier> = RiskEngine::new(vec![
            Box::new(EmptyPoolDeny {}),
            Box::new(StaticPoolDenyList::new()),
            Box::new(UnsupportedPoolList::new()),
        ]);

        while let Ok(()) = latest_block_receiver.changed().await {
            let latest_block = latest_block_receiver.borrow().clone();
            let height = latest_block.header().height;

            // Ensure the lock is dropped to avoid halting the machinery.
            if let Err(err) = tx_tracker.lock().await.observe_block(latest_block) {
                // TODO(xla): Move this to tracker observation impl.
                error!("failed to tracker::observe: {}", err);
            };

            if let Ok((funds, pairs)) = pool_fetcher
                .fetch(lcd.clone(), mekapi.clone(), &account_id.clone(), height)
                .await
            {
                let pools = dex::Pools::from(
                    pairs
                        .into_iter()
                        .filter(|pool| !risk.exclude(pool))
                        .collect::<Vec<_>>(),
                );
                for strategy in strategies.iter_mut() {
                    let trades = strategy.execute(height, &account_id, &funds, &pools).await;

                    for trade in trades {
                        trade_sender.send((height, trade)).await?;
                    }
                }
            }
        }

        Ok::<(), anyhow::Error>(())
    })
    .fuse();

    // TODO(xla): Handle signals.
    loop {
        futures::select! {
            join = metrics_server_handle => join??,
            join = driver_handle => join??,
            join = latest_block_handle => join??,
            join = broadcast_handle => join??,
            join = execution_handle => join??,
            complete => break
        }
    }

    Ok(client.close()?)
}
