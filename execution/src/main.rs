use std::{fs, sync::Arc};

use anyhow::Result;
use futures::{FutureExt as _, StreamExt};
use tendermint_rpc::{
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use tokio::sync::{mpsc, watch, Mutex};

use common::{
    metrics,
    tracing::{self, debug, error},
};
use terrars::{
    dex,
    lcd::{self, Lcd},
    mekapi, BaseAccount,
};

use execution::{
    accounts,
    block_emitter::{self, Emitter as _},
    broadcast, config, params,
    risk_engine::{
        EmptyPoolDeny, RiskClassifier, RiskEngine, StaticPoolAllowList, UnsupportedPoolList,
    },
    strategy::{self, fetcher::Fetcher as _, Trade},
    tracker::{self, Tracker as _},
};

const BINARY_NAMESPACE: &str = "execution";

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let (args, args_router) = argh::from_env::<config::Args>().router();

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
    let (trade_sender, trade_receiver) = mpsc::channel::<Trade>(100);

    let mut registry = metrics::Registry::new(BINARY_NAMESPACE);

    let emitter =
        block_emitter::Observed::new(&mut registry, block_emitter::Rpc::new(latest_block_sender));
    let lcd = execution::metrics::lcd::Observed::new(&mut registry, lcd::Http::new(args.lcd_url)?);
    let signing_key_bytes = {
        let raw = fs::read(args.secret_key_file.clone())?;
        hex::decode(&raw)?
    };
    let account_id = terrars::SigningKey::from_bytes(&signing_key_bytes)
        .unwrap()
        .public_key()
        .account_id(accounts::PREFIX)
        .unwrap();
    // Get latest account information to ensure sequence is consistent.
    let account = lcd.auth_account(&account_id).await?.unwrap_or(BaseAccount {
        account_number: 0,
        address: account_id.to_string(),
        pub_key: None,
        sequence: 0,
    });
    let mekapi = execution::metrics::mekapi::Observed::new(
        &mut registry,
        mekapi::Http::new(args.mekapi_url)?,
    );
    let pool_fetcher =
        strategy::fetcher::Observed::new(&mut registry, strategy::fetcher::Mekapi {});
    let tx_tracker = Arc::new(Mutex::new(tracker::metrics::Observed::new(
        &mut registry,
        tracker::InMemory::default(),
    )));
    let broadcaster = broadcast::Observed::new(
        &mut registry,
        broadcast::Rpc::new(
            client.clone(),
            lcd.clone(),
            args.chain_id,
            account,
            signing_key_bytes,
        ),
    );

    // TODO(xla): Load configured strategies.
    let mut strategies: Vec<Box<dyn strategy::Strategy + Sync>> =
        vec![Box::new(strategy::metrics::Observed::new(
            &mut registry,
            strategy::multi_dex::multi_hop::MultiHop::default(),
        ))];

    let params_server = params::bind(&mut registry, args.params_addr, args_router);
    let mut params_server_handle = tokio::spawn(params_server).fuse();

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

    // Subroutine which compiles, signs and broadcasts message bundles into transactions.
    let mut broadcast_handle = tokio::spawn(broadcast::run(
        trade_receiver,
        broadcaster,
        Arc::clone(&tx_tracker),
        args.execute_trades,
    ))
    .fuse();

    // Subroutine to keep the dependent information for strategies up-to-date and execute
    // strategies.
    let mut execution_handle = tokio::spawn(async move {
        debug!("Init strategy executor");

        let risk: RiskEngine<dyn RiskClassifier> = RiskEngine::new(vec![
            Box::new(EmptyPoolDeny {}),
            Box::new(UnsupportedPoolList::new()),
            Box::new(StaticPoolAllowList::new()),
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
                        trade_sender.send(trade).await?;
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
            join = params_server_handle => join??,
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
