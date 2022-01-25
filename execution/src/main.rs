use futures::StreamExt;
use tokio::sync::watch;

use tendermint_rpc::{
    event::EventData,
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use terra_rust_api::Terra;

use execution::config;
use execution::dex;

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
    env_logger::init();

    // Set up RPC client.
    let (client, driver) = WebSocketClient::new(config::RPC_URL).await?;
    // Advance the internal machinery of the RPC client.
    let driver_handle = tokio::spawn(driver.run());

    // Ensure the data source has caught up to the latest state.
    let status = client.status().await?;
    if status.sync_info.catching_up {
        return Err(Error::NodeCatchingUp);
    }

    // Set up lcd client for state retrieval. This is a naïve and slow way.
    let lcd = Terra::lcd_client_no_tx(config::LCD_URL, config::CHAIN_ID);

    // Fetch liquid UST pairs on Terraswap. This operation is expensive and will take some time.
    let uusd = terraswap::asset::AssetInfo::NativeToken {
        denom: "uusd".to_string(),
    };
    let terraswap_pairs = dex::terraswap::liquid_pairs(&lcd, Some(uusd)).await?;

    // Keep the latest block up-to-date so it can function as barrier for the next iteration of the
    // execution loop. By fetching it first we ensure the watch channel has an initial value.
    let (latest_block_sender, mut latest_block_receiver) = {
        let res = client.latest_block().await?;
        watch::channel(res.block)
    };

    // Subroutine to keep lastest block up-to-date.
    let latest_block_handle = {
        let client = client.clone();
        tokio::spawn(async move {
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
    };

    // Subroutine to keep the dependent information for strategies up-to-date and execute
    // strategies.
    let execution_handle = tokio::spawn(async move {
        while let Ok(_latest_block) = latest_block_receiver.changed().await {
            // Update prices
            // run strategies
            // broadcast txs
        }

        Ok::<(), Error>(())
    });

    println!("Liquid pairs: {}", terraswap_pairs.len());

    // TODO(xla): Handle signals.
    // TODO(xla): Select on all long-running futures.

    client.close()?;
    driver_handle.await??;
    latest_block_handle.await??;
    execution_handle.await??;

    Ok(())
}
