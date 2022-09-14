use std::sync::Arc;

use chainrs::cosmos::{auth::BaseAccount, base::AccountId};
use common::{
    tracing::{debug, error},
    Result,
};
use cosmwasm_bignumber::{Decimal256, Uint256};
use futures::StreamExt as _;
use lcd::Lcd;
use strategy::{Bundle, Strategy, Token};
use tendermint::block;
use tendermint_rpc::{
    query::{EventType, Query},
    SubscriptionClient as _,
    WebSocketClient,
};
use tokio::sync::{mpsc, watch, Mutex};
use zenith_rs::{AuctionResponse, Builder};

use crate::{block_emitter::Emitter, fetcher::Fetcher, tracker::Tracker};

/// Subroutine to keep the dependent information for strategies up-to-date and execute
/// strategies.
#[allow(clippy::too_many_arguments)]
pub async fn execution<F, L, M, T>(
    lcd: L,
    mekapi: M,
    fetcher: F,
    tx_tracker: Arc<Mutex<T>>,
    mut latest_block_receiver: watch::Receiver<block::Block>,
    mut mempool_tx_receiver: mpsc::Receiver<mekapi::v1::MempoolTxV5Message>,
    bundle_sender: mpsc::Sender<Bundle>,
    proxy_addr: AccountId,
    _account_id: AccountId,
    mut strategies: Vec<Box<dyn Strategy + Sync>>,
) -> Result<()>
where
    F: Fetcher<L, M>,
    L: Lcd + Clone,
    M: mekapi::pools::Api + Clone,
    T: Tracker,
{
    debug!("Init strategy executor");

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
                    let pools = pairs
                            .into_iter()
                            .filter(|p| p.assets[0].weight == p.assets[1].weight) // XXX
                            .filter(|p| p.assets[0].amount > Decimal256::one() && p.assets[1].amount > Decimal256::one())
                            .collect::<Vec<_>>();
                    let mut bal = cosmwasm_bignumber::Decimal256::from_ratio(*funds.get(&Token { id: "uosmo".to_owned(), decimals: 6, native: true }).unwrap_or(&Uint256::zero()), 10u64.pow(6u32));
                    // XXX: remove 1.0 because we're paying for gas, so we can't use our full
                    //      balance.
                    bal = if bal < Decimal256::one() { bal } else { bal - Decimal256::one() };
                    for strategy in strategies.iter_mut() {
                        strategy
                            .execute_pools(height, bal, &pools)
                            .await;
                    }
                }
            },

            Some(msg) = mempool_tx_receiver.recv() => {
                let height = <tendermint::block::Height as Into<u64>>::into(msg.height) + 1;
                for strategy in strategies.iter_mut() {
                    let trades = strategy.execute_mempool_tx(msg.height, &msg.pools.clone().into_iter().flat_map(TryInto::try_into).collect::<Vec<_>>()).await;

                    for trade in trades {
                        let bundle = Bundle {
                            height,
                            trade,
                            trade_index: 0,
                            txs: vec![msg.tx.clone()],
                        };
                        bundle_sender.send(bundle).await?;
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

pub async fn base_account(
    lcd: impl Lcd,
    account_id: AccountId,
    acc: Arc<Mutex<BaseAccount>>,
    mut latest_block_receiver: watch::Receiver<block::Block>,
) -> Result<()> {
    debug!("Init base account");
    loop {
        tokio::select! {
            Ok(()) = latest_block_receiver.changed() => {
                let r = lcd.auth_account(&account_id).await;
                match r {
                    Err(e) => error!("failed to get account {:?}: {:?}", account_id, e),
                    Ok(None) => error!("no account data found for {:}", account_id),
                    Ok(Some(a)) => {
                        debug!("account state {:?}", a);
                        let mut c = acc.lock().await;
                        *c = a;
                    },
                }
            },
            else => break,
        }
    }
    Ok(())
}

pub async fn auctions(
    builder: impl Builder,
    auction: watch::Sender<AuctionResponse>,
    mut latest_block_receiver: watch::Receiver<block::Block>,
) -> Result<()> {
    debug!("Init auctions");
    loop {
        tokio::select! {
            Ok(()) = latest_block_receiver.changed() => {
                let height: u64;
                {
                    let latest_block = latest_block_receiver.borrow().clone();
                    height = latest_block.header().height.into();
                }
                let r = builder.auction("osmosis-1".to_string(), height + 1).await;
                match r {
                    Err(e) => debug!("no auction for {:?}: {:?}", height, e),
                    Ok(a) => {
                        debug!("found an auction for {:?}", a);
                        auction.send(a)?;
                    },
                }
            },
            else => break,
        }
    }
    Ok(())
}

pub async fn mempool_tx_v5<M>(
    mekapi: M,
    mempool_tx_sender: mpsc::Sender<mekapi::v1::MempoolTxV5Message>,
) -> Result<()>
where
    M: mekapi::v1::Api,
{
    debug!("Init mempool tx stream");

    let mut events = mekapi.v1_events_mempooltx_v5().await?;

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
                events = mekapi.v1_events_mempooltx_v5().await?;
            },
        }
    }
}
