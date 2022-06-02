use std::sync::Arc;

use chainrs::cosmos::base::AccountId;
use common::{tracing::debug, Result};
use cosmwasm_bignumber::Uint256;
use lcd::Lcd;
use strategy::{Strategy, Trade};
use tendermint::block;
use tokio::sync::{mpsc, watch, Mutex};

use crate::{tracker::Tracker, venue::Fetcher};

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
    proxy_addr: AccountId,
    account_id: AccountId,
    mut strategies: Vec<Box<dyn Strategy + Sync>>,
) -> Result<()>
where
    F: Fetcher<L, M>,
    L: Lcd + Clone,
    M: mekapi::v3::Api + Clone,
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
                            // .filter(|pool| !risk.exclude(pool))
                            .collect::<Vec<_>>();
                    for strategy in strategies.iter_mut() {
                        let bal = cosmwasm_bignumber::Decimal256::from_ratio(*funds.get("uusd").unwrap_or(&Uint256::zero()), 10u64.pow(6u32));
                        strategy
                            .execute_pools(height, bal, &pools)
                            .await;
                    }
                }
            },

            Some(msg) = mempool_tx_receiver.recv() => {
                for strategy in strategies.iter_mut() {
                    let trades = strategy.execute_mempool_tx(msg.height, &msg.pools.iter().map(Into::into).collect::<Vec<_>>()).await;

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
