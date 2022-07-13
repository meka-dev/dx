use std::{fs, str::FromStr, sync::Arc, time::Duration};

use axum::{Router, Server};
use backoff::{future::retry, ExponentialBackoffBuilder};
use chainrs::cosmos::{auth::BaseAccount, base::AccountId};
use common::{
    err,
    metrics,
    params,
    signals,
    tracing::{self, error},
    Result,
};
use cosmrs::crypto::secp256k1::SigningKey;
use execution::{
    accounts,
    block_emitter,
    broadcast,
    config,
    fetcher,
    routines,
    tracker,
    treasurer,
};
use futures::FutureExt as _;
use lcd::{self, Lcd};
use strategy::{self, Token, Trade};
use tendermint_rpc::{Client as _, SubscriptionClient as _, WebSocketClient};
use tokio::sync::{mpsc, watch, Mutex};

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let (args, args_routes) = argh::from_env::<config::Args>().router();
    let addresses = &(*address_book::CURRENT);

    // Set up RPC client.
    let back = ExponentialBackoffBuilder::new()
        .with_max_interval(Duration::from_secs(20))
        .build();
    let (client, driver) = retry(back.clone(), || async {
        match WebSocketClient::new(args.rpc_url.as_str()).await {
            Ok(t) => Ok(t),
            Err(e) => {
                error!("could not connect to node: {}", e);
                Err(backoff::Error::transient(e))
            },
        }
    })
    .await?;
    // Advance the internal machinery of the RPC client.
    let mut driver_handle = tokio::spawn(driver.run()).fuse();

    // Ensure the data source has caught up to the latest state.
    retry(back.clone(), || async {
        if client
            .status()
            .await
            .map_err(|e| err!("failed to get node status: {:}", e))?
            .sync_info
            .catching_up
        {
            error!("node still catching up");
            return Err(backoff::Error::transient(err!("node still catching up")));
        };
        Ok(())
    })
    .await?;

    let (shutdown_sender, _shutdown_receiver) = watch::channel(false);

    // Keep the latest block up-to-date so it can function as barrier for the next iteration of the
    // execution loop. By fetching it first we ensure the watch channel has an initial value.
    let (latest_block_sender, latest_block_receiver) = retry(back, || async {
        match client.latest_block().await {
            Ok(res) => Ok(watch::channel(res.block)),
            Err(e) => {
                error!("failed to poll latest block: {}", e);
                Err(backoff::Error::transient(e))
            },
        }
    })
    .await?;

    let (mempool_tx_sender, mempool_tx_receiver) =
        mpsc::channel::<mekapi::v1::MempoolTxV5Message>(1024);
    let (trade_sender, trade_receiver) = mpsc::channel::<Trade>(100);

    let mut registry = metrics::Registry::new(config::BINARY);

    let emitter =
        block_emitter::Observed::new(&mut registry, block_emitter::Rpc::new(latest_block_sender));
    let lcd = execution::metrics::lcd::Observed::new(&mut registry, lcd::Http::new(args.lcd_url)?);
    let signing_key_bytes = {
        let raw = fs::read(args.secret_key_file.clone())?;
        hex::decode(&raw)?
    };
    let account_id = AccountId::from_str(
        SigningKey::from_bytes(&signing_key_bytes)
            .unwrap()
            .public_key()
            .account_id(accounts::PREFIX)
            .unwrap()
            .as_ref(),
    )?;
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
    let pool_fetcher = fetcher::Observed::new(&mut registry, fetcher::Mekapi {});
    let tx_tracker = Arc::new(Mutex::new(tracker::metrics::Observed::new(
        &mut registry,
        tracker::InMemory::default(),
    )));
    let broadcaster = broadcast::Observed::new(
        &mut registry,
        broadcast::Rpc::new(
            client.clone(),
            args.chain_id,
            account,
            signing_key_bytes,
            Some(addresses.treasury.clone()),
        ),
    );
    let treasurer = treasurer::Treasurer::new(
        &mut registry,
        addresses.clone(),
        Arc::clone(&args.execute_trades),
        lcd.clone(),
    );

    let strategies: Vec<Box<dyn strategy::Strategy + Sync>> =
        vec![Box::new(execution::metrics::strategy::Observed::new(
            &mut registry,
            strategy::multi_dex::multi_hop::MultiHop::new(Token {
                id: "uosmo".to_owned(),
                decimals: 6,
                native: true,
            }),
        ))];

    let mut signals_handle = tokio::spawn(signals::routine(shutdown_sender)).fuse();

    let router = Router::new()
        .nest("/params", params::routes(&mut registry, args_routes))
        .nest("/metrics", metrics::routes(registry, config::BINARY));
    let mut op_server_handle =
        tokio::spawn(Server::bind(&args.op_addr).serve(router.into_make_service())).fuse();

    let mut treasurer_handle =
        tokio::spawn(treasurer::run(treasurer, latest_block_receiver.clone())).fuse();

    let mut latest_block_handle =
        tokio::spawn(routines::latest_block(client.clone(), emitter)).fuse();

    let mut mempool_tx_handle =
        tokio::spawn(routines::mempool_tx_v5(mekapi.clone(), mempool_tx_sender)).fuse();

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
        addresses.arby.clone(),
        account_id,
        strategies,
    ))
    .fuse();

    loop {
        tokio::select! {
            join = &mut signals_handle => {
                join??;
                break;
            },
            join = &mut op_server_handle => join??,
            join = &mut driver_handle => join??,
            join = &mut treasurer_handle => join??,
            join = &mut latest_block_handle => join??,
            join = &mut mempool_tx_handle => join??,
            join = &mut broadcast_handle => join??,
            join = &mut execution_handle => join??,

            else => break
        }
    }

    Ok(client.close()?)
}
