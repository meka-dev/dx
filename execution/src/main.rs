// TODO(xla): Set up logging/tracing.
// TODO(xla): Set up telemetry (Prometheus): process, futures, strategies
// TODO(xla): Move from lcd to direct RPC calls to support height queries.
// TODO(xla): Implement swaps for DEXes to produce proper messages.
// TODO(xla): Bundle messages into txs and broadcast.
// TODO(xla): Review depdencies and remove duplications.
// TODO(xla): Extend telemetry with
//              * fetch durations (pools, accounts)
//              * current account balances

#![allow(clippy::or_fun_call)]

use std::{fs, ops::Sub, sync::Arc, time::Instant};

use futures::{FutureExt as _, StreamExt};
use tokio::sync::watch;

use tendermint_rpc::{
    event::EventData,
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use terrars::Lcd;

use execution::{accounts, config, dex, metrics, strategy};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error("Node is still catching up")]
    NodeCatchingUp,
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Rpc(#[from] tendermint_rpc::Error),
    #[error(transparent)]
    Terrars(#[from] terrars::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up logging.
    env_logger::init();

    let args: config::Args = argh::from_env();

    // Set up metrics endpoint.
    let (metrics, metrics_server) = metrics::Metrics::new(args.metrics_addr);
    let mut metrics_server_handle = tokio::spawn(metrics_server).fuse();

    let secret_key = {
        let raw = fs::read(args.secret_key_file)?;
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
    let lcd = Lcd::lcd_client_no_tx(args.lcd_url, args.chain_id);

    let mut token_cache = dex::TokenCache::default();
    let mut pools = {
        log::debug!("Fetching DEX pairs with pool info");

        // Fetch liquid UST pairs on Astroprot and Terraswap. This operation is expensive and will
        // take some time.
        let filter = Some(dex::Token::ust());

        let start = Instant::now();
        let mut pairs =
            dex::astroport::liquid_pairs(&lcd, &mut token_cache, filter.clone()).await?;
        pairs.append(
            &mut dex::terraswap::liquid_pairs(&lcd, &mut token_cache, filter.clone()).await?,
        );
        pairs.append(&mut dex::loopmarkets::liquid_pairs(&lcd, &mut token_cache, filter).await?);
        log::debug!(
            "Fetched {} pools in {:?}",
            pairs.len(),
            Instant::now().sub(start)
        );

        dex::Pools::from(pairs)
    };

    // Set up RPC client.
    let (client, driver) = WebSocketClient::new(args.rpc_url.as_str()).await?;
    // Advance the internal machinery of the RPC client.
    let mut driver_handle = tokio::spawn(driver.run()).fuse();

    // Ensure the data source has caught up to the latest state.
    let status = client.status().await?;
    if status.sync_info.catching_up {
        return Err(Error::NodeCatchingUp);
    }

    log::info!("Node status: {:?}", status);

    // Keep the latest block up-to-date so it can function as barrier for the next iteration of the
    // execution loop. By fetching it first we ensure the watch channel has an initial value.
    let (latest_block_sender, mut latest_block_receiver) = {
        let res = client.latest_block().await?;
        watch::channel(res.block)
    };

    // Subroutine to keep lastest block up-to-date.
    let mut latest_block_handle = {
        let client = client.clone();
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
                    latest_block_sender.send(block).unwrap();
                }
            }

            Ok::<(), Error>(())
        })
        .fuse()
    };

    // Subroutine to keep the dependent information for strategies up-to-date and execute
    // strategies.
    let mut execution_handle = tokio::spawn(async move {
        log::debug!("Init strategy executor");

        // TODO(xla): Load configured strategies.
        let strategies: Vec<Box<dyn strategy::Strategy + Sync>> =
            vec![Box::new(strategy::metrics::Observed::new(
                config::VERSION.to_string(),
                Arc::clone(&metrics),
                strategy::multi_dex::SinglePair::default(),
            ))];

        while let Ok(()) = latest_block_receiver.changed().await {
            let latest_block = latest_block_receiver.borrow().clone();
            log::debug!(
                "Fetching latest pair prices for {} pools at height {}",
                pools.0.len(),
                latest_block.header.height,
            );

            // Ensure latest prices of pairs.
            let start = Instant::now();
            // TODO(xla): Parallelise with batches modulo num cpus.
            // TODO(xla): Query at consistent block height.
            for pool in pools.0.iter_mut() {
                pool.update(&lcd, &mut token_cache).await?;
            }

            log::debug!(
                "Updated {} in {:?}",
                pools.0.len(),
                Instant::now().sub(start)
            );

            let funds = accounts::fetch(&lcd, &account_id).await?;

            for strategy in &strategies {
                let trades = strategy
                    .execute(latest_block.header.height, &account_id, &funds, &pools)
                    .await;

                for _trade in &trades {
                    // broadcast tx
                    // lcd.submit_transaction_sync(&secp, &private_key, &messages, None).await?;
                }
            }
        }

        Ok::<(), Error>(())
    })
    .fuse();

    // TODO(xla): Handle signals.
    loop {
        futures::select! {
            join = metrics_server_handle => join??,
            join = driver_handle => join??,
            join = latest_block_handle => join??,
            join = execution_handle => join??,
            complete => break
        }
    }

    client.close().map_err(Error::from)
}
