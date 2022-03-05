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
    event::EventData,
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use tokio::sync::{mpsc, watch, Mutex};

use terrars::{
    dex,
    lcd::{self, Lcd},
    mekapi, BaseAccount, Message, Msg as _,
};

use execution::{
    accounts, broadcast, config, metrics,
    risk_engine::{EmptyPoolDeny, RiskClassifier, RiskEngine, StaticPoolDenyList},
    strategy::{self, fetcher::Fetcher as _, Trade},
    tracker::{self, Tracker},
};

#[tokio::main]
async fn main() -> Result<()> {
    // Set up logging.
    env_logger::init();

    let args: config::Args = argh::from_env();

    // Set up metrics endpoint.
    let (metrics, metrics_server) = metrics::Metrics::new(args.metrics_addr);
    let mut metrics_server_handle = tokio::spawn(metrics_server).fuse();

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

    // Set up lcd client for state retrieval. This is a naïve and slow way.
    let lcd = metrics::lcd::Observed::new(Arc::clone(&metrics), lcd::Http::new(args.lcd_url)?);
    let mekapi =
        metrics::mekapi::Observed::new(Arc::clone(&metrics), mekapi::Http::new(args.mekapi_url)?);

    // Set up RPC client.
    let (client, driver) = WebSocketClient::new(args.rpc_url.as_str()).await?;
    // Advance the internal machinery of the RPC client.
    let mut driver_handle = tokio::spawn(driver.run()).fuse();

    // Ensure the data source has caught up to the latest state.
    let status = client.status().await?;
    if status.sync_info.catching_up {
        log::error!("Node status: {:?}", status);
        return Err(anyhow::anyhow!("node still catching up"));
    }

    // Keep the latest block up-to-date so it can function as barrier for the next iteration of the
    // execution loop. By fetching it first we ensure the watch channel has an initial value.
    let (latest_block_sender, mut latest_block_receiver) = {
        let res = client.latest_block().await?;
        watch::channel(res.block)
    };

    // Subroutine to keep lastest block up-to-date.
    let mut latest_block_handle = {
        let client = client.clone();
        let metrics = Arc::clone(&metrics);
        tokio::spawn(async move {
            log::debug!("Init block subscription");

            let query = Query::from(EventType::NewBlock);
            let mut events = client.subscribe(query).await?;

            while let Some(res) = events.next().await {
                let event = res.unwrap();

                if let EventData::NewBlock {
                    block: Some(block), ..
                } = event.data
                {
                    let height = block.header().height.value();
                    latest_block_sender.send(block).unwrap();

                    metrics
                        .lock()
                        .await
                        .latest_block_height
                        .get_or_create(&metrics::LatestHeightLabels {
                            version: config::VERSION.to_string(),
                        })
                        .set(height);
                }
            }

            Ok::<(), anyhow::Error>(())
        })
        .fuse()
    };

    let tx_tracker = Arc::new(Mutex::new(tracker::InMemory::default()));
    let (broadcast_sender, mut broadcast_receiver) = mpsc::channel::<(block::Height, Trade)>(100);

    // Subroutine which compiles, signs and broadcasts message bundles into transactions.
    let mut broadcast_handle = {
        let account_id = account_id.clone();
        let client = client.clone();
        let lcd = lcd.clone();
        let metrics = Arc::clone(&metrics);
        let tx_tracker = Arc::clone(&tx_tracker);

        tokio::spawn(async move {
            log::debug!("Init broadcaster");

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

            while let Some((_height, trade)) = broadcast_receiver.recv().await {
                let route = trade.route;
                if let Some(addr) = (*tx_tracker.lock().await).is_tracked(&route) {
                    log::debug!("pool {} in route  ", addr);
                    continue;
                }

                let msgs: Vec<Message> = trade
                    .messages
                    .into_iter()
                    .map(|msg| msg.to_any().unwrap())
                    .collect();

                let simulate_tx = terrars::tx::sign(
                    args.chain_id.clone(),
                    // FIXME(xla): Proper error handling.
                    &terrars::SigningKey::from_bytes(&secret_key_bytes).unwrap(),
                    account.account_number,
                    sequence,
                    0,
                    0,
                    Height::try_from(0_u64).unwrap(),
                    msgs.clone(),
                )?;

                let expected_profit = (trade.expected_profit.0 / 10u64.pow(12).into()).as_u64();

                if let Ok(gas_info) = broadcast::simulate(
                    &lcd.clone(),
                    Arc::clone(&metrics),
                    &simulate_tx,
                    msgs.len(),
                    expected_profit,
                )
                .await
                {
                    let fee = (gas_info.gas_used as f64 * 0.18) as u64;
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
                            (gas_info.gas_used as f64 * 1.05) as u64,
                            Height::try_from(0_u64).unwrap(),
                            msgs,
                        )?;
                        match client.broadcast_tx_sync(Transaction::from(tx)).await {
                            Ok(check) => {
                                if check.code.is_ok() {
                                    sequence += 1;

                                    let _ = (*tx_tracker.lock().await).track(check.hash, route);
                                }
                                log::info!(
                                    "msg=submit hash={} code={:?} log={}",
                                    check.hash,
                                    check.code,
                                    check.log
                                );
                            },
                            Err(err) => log::error!("tx submit failed: {}", err),
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
        log::debug!("Init strategy executor");

        let risk: RiskEngine<dyn RiskClassifier> = RiskEngine::new(vec![
            Box::new(EmptyPoolDeny {}),
            Box::new(StaticPoolDenyList::new()),
        ]);

        let fetcher =
            strategy::fetcher::Observed::new(Arc::clone(&metrics), strategy::fetcher::Mekapi {});

        // TODO(xla): Load configured strategies.
        let strategies: Vec<Box<dyn strategy::Strategy + Sync>> =
            vec![Box::new(strategy::metrics::Observed::new(
                Arc::clone(&metrics),
                strategy::multi_dex::SinglePair::default(),
            ))];

        while let Ok(()) = latest_block_receiver.changed().await {
            let latest_block = latest_block_receiver.borrow().clone();
            let height = latest_block.header().height;

            // Ensure the lock is dropped to avoid halting the machinery.
            if let Err(err) = tx_tracker.lock().await.observe(latest_block) {
                // TODO(xla): Move this to tracker observation impl.
                log::error!("failed to tracker::observe: {}", err);
            };

            if let Ok((funds, pairs)) = fetcher
                .fetch(lcd.clone(), mekapi.clone(), &account_id.clone(), height)
                .await
            {
                let pools = dex::Pools::from(
                    pairs
                        .into_iter()
                        .filter(|pool| !risk.exclude(pool))
                        .collect::<Vec<_>>(),
                );
                for strategy in &strategies {
                    let trades = strategy.execute(height, &account_id, &funds, &pools).await;

                    for trade in trades {
                        broadcast_sender.send((height, trade)).await?;
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
