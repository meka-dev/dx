use std::sync::Arc;

use common::{
    tracing::{debug, error},
    Result,
};
use futures::StreamExt as _;
use tendermint::block;
use tendermint_rpc::{
    query::{EventType, Query},
    SubscriptionClient as _,
    WebSocketClient,
};
use terrars::{dex, lcd::Lcd};
use tokio::sync::{mpsc, watch, Mutex};

use crate::{
    block_emitter::Emitter,
    risk_engine::{
        EmptyPoolDeny,
        RiskClassifier,
        RiskEngine,
        StaticPoolAllowList,
        UnsupportedPoolList,
    },
    strategy::{fetcher::Fetcher, Strategy, Trade},
    tracker::Tracker,
};

/// Subroutine to keep the dependent information for strategies up-to-date and execute
/// strategies.
#[allow(clippy::too_many_arguments)]
pub async fn execution<F, L, M, T>(
    lcd: L,
    mekapi: M,
    fetcher: F,
    tx_tracker: Arc<Mutex<T>>,
    mut latest_block_receiver: watch::Receiver<block::Block>,
    mut mempool_tx_receiver: mpsc::Receiver<mekapi::v1::MempoolTxV2Message>,
    trade_sender: mpsc::Sender<Trade>,
    proxy_addr: terrars::AccountId,
    account_id: terrars::AccountId,
    mut strategies: Vec<Box<dyn Strategy + Sync>>,
) -> Result<()>
where
    F: Fetcher<L, M>,
    L: Lcd + Clone,
    M: mekapi::v3::Api + Clone,
    T: Tracker,
{
    debug!("Init strategy executor");

    let risk: RiskEngine<dyn RiskClassifier> = RiskEngine::new(vec![
        Box::new(EmptyPoolDeny {}),
        Box::new(StaticPoolAllowList::new()),
        Box::new(UnsupportedPoolList::new()),
    ]);

    loop {
        tokio::select! {
            Ok(()) = latest_block_receiver.changed() => {
                let latest_block = latest_block_receiver.borrow().clone();
                let height = latest_block.header().height;

                // Ensure the lock is dropped to avoid halting the machinery.
                let _ = tx_tracker.lock().await.observe_block(latest_block);

                if let Ok((funds, pairs)) = fetcher
                    .fetch(lcd.clone(), mekapi.clone(), &proxy_addr, height)
                    .await
                {
                    let pools = dex::Pools::from(
                        pairs
                            .into_iter()
                            .filter(|pool| !risk.exclude(pool))
                            .collect::<Vec<_>>(),
                    );
                    for strategy in strategies.iter_mut() {
                        let trades = strategy
                            .execute_pools(height, &account_id, &funds, &pools)
                            .await;

                        for trade in trades {
                            trade_sender.send(trade).await?;
                        }
                    }
                }
            },

            Some(msg) = mempool_tx_receiver.recv() => {
                for strategy in strategies.iter_mut() {
                    let trades = strategy.execute_mempool_tx(msg.height, &account_id, &msg.pools).await;

                    for trade in trades {
                        trade_sender.send(trade).await?;
                    }
                }
            }

            else => break,
        }
    }

    Ok(())
}

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
