#![allow(clippy::or_fun_call)]

use std::{env, ops::Sub, time::Instant};

use futures::FutureExt as _;
use futures::StreamExt;
use tokio::sync::watch;

use tendermint_rpc::{
    event::EventData,
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use terrars::Lcd;

use execution::config;
use execution::dex;
use execution::strategy;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error("Node is still catching up")]
    NodeCatchingUp,
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

    // TODO(xla): Safely load private key.

    // Set up lcd client for state retrieval. This is a naïve and slow way.
    let lcd_url: &str = &env::var("LCD_URL").unwrap_or(config::LCD_URL.to_string());
    let chain_id: &str = &env::var("CHAIN_ID").unwrap_or(config::CHAIN_ID.to_string());
    let lcd = Lcd::lcd_client_no_tx(lcd_url, chain_id);

    let mut token_cache = dex::TokenCache::default();
    let mut pools = {
        log::debug!("Fetching DEX pairs with pool info");

        // Fetch liquid UST pairs on Astroprot and Terraswap. This operation is expensive and will
        // take some time.
        let filter = Some(dex::Token::Native {
            denom: dex::DENOM_UST.to_string(),
        });

        let start = Instant::now();
        let mut pairs =
            dex::astroport::liquid_pairs(&lcd, &mut token_cache, filter.clone()).await?;
        pairs.append(&mut dex::terraswap::liquid_pairs(&lcd, &mut token_cache, filter).await?);

        log::debug!(
            "Fetched {} pools in {:?}",
            pairs.len(),
            Instant::now().sub(start)
        );

        dex::Pools::from(pairs)
    };

    // Set up RPC client.
    let rpc_url: &str = &env::var("RPC_URL").unwrap_or(config::RPC_URL.to_string());
    let (client, driver) = WebSocketClient::new(rpc_url).await?;
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
        let strategies: Vec<Box<dyn strategy::Strategy>> =
            vec![Box::new(strategy::multi_dex::SinglePair::default())];

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

            for strategy in &strategies {
                let _messages = strategy.execute(&pools);

                // broadcast tx
                // lcd.submit_transaction_sync(&secp, &private_key, &messages, None).await?;
            }
        }

        Ok::<(), Error>(())
    })
    .fuse();

    // TODO(xla): Handle signals.
    loop {
        futures::select! {
            join = driver_handle => join??,
            join = latest_block_handle => join??,
            join = execution_handle => join??,
            complete => break
        }
    }

    client.close().map_err(Error::from)
}
