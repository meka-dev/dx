use common::{
    tracing::{debug, error},
    Result,
};
use futures::StreamExt as _;
use tendermint_rpc::{
    query::{EventType, Query},
    SubscriptionClient as _,
    WebSocketClient,
};
use tokio::sync::mpsc;

use crate::block_emitter::Emitter;

pub async fn latest_block<E>(client: WebSocketClient, emitter: E) -> Result<()>
where
    E: Emitter,
{
    debug!("Init block subscription");

    let query = Query::from(EventType::NewBlock);
    let mut events = client.subscribe(query).await?;

    while let Some(res) = events.next().await {
        if let Err(err) = emitter.emit(res) {
            error!("failed to emit latest block: {}", err);
        }
    }

    Ok(())
}

pub async fn mempool_tx<M>(
    mekapi: M,
    mempool_tx_sender: mpsc::Sender<mekapi::v1::MempoolTxMessage>,
) -> Result<()>
where
    M: mekapi::v1::Api,
{
    debug!("Init mempool tx stream");

    let mut events = mekapi.v1_events_mempooltx().await?;

    loop {
        match events.next().await {
            Some(res) => match res {
                Ok(msg) => {
                    if let Err(err) = mempool_tx_sender.try_send(msg) {
                        error!("failed to send mempool tx: {}", err);
                    }
                },
                Err(err) => error!("failed to decode mempool tx: {}", err),
            },
            None => {
                events = mekapi.v1_events_mempooltx().await?;
            },
        }
    }
}

pub async fn mempool_tx_v2<M>(
    mekapi: M,
    mempool_tx_sender: mpsc::Sender<mekapi::v1::MempoolTxV2Message>,
) -> Result<()>
where
    M: mekapi::v1::Api,
{
    debug!("Init mempool tx stream");

    let mut events = mekapi.v1_events_mempooltx_v2().await?;

    loop {
        match events.next().await {
            Some(res) => match res {
                Ok(msg) => {
                    if let Err(err) = mempool_tx_sender.try_send(msg) {
                        error!("failed to send mempool tx: {}", err);
                    }
                },
                Err(err) => error!("failed to decode mempool tx: {}", err),
            },
            None => {
                events = mekapi.v1_events_mempooltx_v2().await?;
            },
        }
    }
}

pub async fn mempool_tx_v4<M>(
    mekapi: M,
    mempool_tx_sender: mpsc::Sender<mekapi::pools::V4Response>,
) -> Result<()>
where
    M: mekapi::v1::Api,
{
    debug!("Init mempool tx stream");

    let mut events = mekapi.v1_events_mempooltx_v4().await?;

    loop {
        match events.next().await {
            Some(res) => match res {
                Ok(msg) => {
                    if let Err(err) = mempool_tx_sender.try_send(msg) {
                        error!("failed to send mempool tx: {}", err);
                    }
                },
                Err(err) => error!("failed to decode mempool tx: {}", err),
            },
            None => {
                events = mekapi.v1_events_mempooltx_v4().await?;
            },
        }
    }
}
