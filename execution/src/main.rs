use std::{fs, sync::Arc};

use anyhow::Result;
use common::{
    metrics,
    tracing::{self, debug, error},
};
use execution::{
    accounts,
    block_emitter::{self, Emitter as _},
    broadcast,
    config,
    params,
    routines,
    strategy::{self, Trade},
    tracker,
};
use futures::{FutureExt as _, StreamExt as _};
use tendermint_rpc::{
    query::{EventType, Query},
    Client as _,
    SubscriptionClient,
    WebSocketClient,
};
use terrars::{
    lcd::{self, Lcd},
    mekapi,
    BaseAccount,
};
use tokio::sync::{mpsc, watch, Mutex};

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
    let (latest_block_sender, latest_block_receiver) = {
        let res = client.latest_block().await?;
        watch::channel(res.block)
    };
    let (mempool_tx_sender, mempool_tx_receiver) =
        mpsc::channel::<mekapi::v1::MempoolTxV2Message>(1024);
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
        broadcast::Rpc::new(client.clone(), args.chain_id, account, signing_key_bytes),
    );

    // TODO(xla): Load configured strategies.
    let strategies: Vec<Box<dyn strategy::Strategy + Sync>> =
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

    let mut mempool_tx_handle =
        tokio::spawn(routines::mempool_tx_v2(mekapi.clone(), mempool_tx_sender)).fuse();

    // Subroutine which compiles, signs and broadcasts message bundles into transactions.
    let mut broadcast_handle = tokio::spawn(broadcast::run(
        trade_receiver,
        broadcaster,
        Arc::clone(&tx_tracker),
        args.execute_trades,
    ))
    .fuse();

    let mut execution_handle = tokio::spawn(routines::execution(
        lcd,
        mekapi,
        pool_fetcher,
        tx_tracker,
        latest_block_receiver,
        mempool_tx_receiver,
        trade_sender,
        account_id,
        strategies,
    ))
    .fuse();

    // TODO(xla): Handle signals.
    loop {
        tokio::select! {
            join = &mut params_server_handle => join??,
            join = &mut metrics_server_handle => join??,
            join = &mut driver_handle => join??,
            join = &mut latest_block_handle => join??,
            join = &mut mempool_tx_handle => join??,
            join = &mut broadcast_handle => join??,
            join = &mut execution_handle => join??,

            else => break
        }
    }

    Ok(client.close()?)
}
