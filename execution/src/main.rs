// TODO(xla): Set up logging/tracing.
// TODO(xla): Set up telemetry (Prometheus): process, futures
// TODO(xla): Extend telemetry with
//              * fetch durations (pools, accounts)
//              * current account balances

#![allow(clippy::or_fun_call)]

use std::{fs, ops::Sub, sync::Arc, time::Instant};

use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;
use futures::{FutureExt as _, StreamExt};
use tendermint::{
    abci::Transaction,
    block::{self, Height},
};
use tendermint_rpc::{
    event::EventData,
    query::{EventType, Query},
    Client as _, SubscriptionClient, WebSocketClient,
};
use tokio::sync::{mpsc, watch};

use terrars::{
    dex,
    lcd::{self, Lcd},
    mekapi::{self, Mekapi as _},
    tx, BaseAccount, Coin, Message, Msg as _,
};

use execution::{
    accounts, broadcast, config, metrics,
    risk_engine::{RiskClassifier, RiskEngine, StaticPoolDenyList},
    strategy::{self, Trade},
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[error("Broadcast routine has vanished")]
    BroadcasterGone,
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
        let raw = fs::read(args.secret_key_file.clone())?;
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
    let lcd = metrics::lcd::Observed::new(Arc::clone(&metrics), lcd::Http::new(args.lcd_url)?);
    let mekapi =
        metrics::mekapi::Observed::new(Arc::clone(&metrics), mekapi::Http::new(args.mekapi_url)?);

    let deny_list = StaticPoolDenyList::new();
    let risk: RiskEngine<
        execution::metrics::lcd::Observed<lcd::Http>,
        dyn RiskClassifier<execution::metrics::lcd::Observed<lcd::Http>>,
    > = RiskEngine::new(vec![deny_list]);

    let mut token_cache = dex::TokenCache::default();
    let mut pools = {
        log::info!("Fetching DEX pairs with pool info");

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
        log::info!(
            "Fetched {} pools in {:?}",
            pairs.len(),
            Instant::now().sub(start)
        );

        let ps = dex::Pools::from(
            pairs
                .into_iter()
                .filter(|p| !risk.exclude(p.as_ref()))
                .collect::<Vec<_>>(),
        );
        log::debug!("Left with {} pools after risk engine", ps.0.len(),);
        ps
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
        let metrics = Arc::clone(&metrics);
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
                    let height = block.header().height.value();
                    latest_block_sender.send(block).unwrap();

                    metrics
                        .lock()
                        .await
                        .latest_block_height
                        .get_or_create(&metrics::LatestHeightLabels {
                            version: config::VERSION.to_string(),
                        })
                        .set(height);
                }
            }

            Ok::<(), Error>(())
        })
        .fuse()
    };

    let (broadcast_sender, mut broadcast_receiver) = mpsc::channel::<(block::Height, Trade)>(100);

    // Subroutine which compiles, signs and broadcasts message bundles into transactions.
    let mut broadcast_handle = {
        let account_id = account_id.clone();
        let client = client.clone();
        let lcd = lcd.clone();
        let metrics = Arc::clone(&metrics);

        tokio::spawn(async move {
            log::debug!("Init broadcaster");

            let secret_key_bytes = {
                let raw = fs::read(args.secret_key_file.clone())?;
                hex::decode(&raw)?
            };

            // Get latest account information to ensure sequence is consistent.
            let account = lcd.auth_account(&account_id).await?.unwrap_or(BaseAccount {
                account_number: 0,
                address: account_id.to_string(),
                pub_key: None,
                sequence: 0,
            });
            let mut sequence = account.sequence;

            while let Some((
                height,
                Trade {
                    expected_profit,
                    messages,
                    ..
                },
            )) = broadcast_receiver.recv().await
            {
                let messages_len = messages.len();
                let msgs: Vec<Message> = messages
                    .into_iter()
                    .map(|msg| msg.to_any().unwrap())
                    .collect();

                // TODO(xla): Fetch gas prices.
                let tx = terrars::tx::sign(
                    args.chain_id.clone(),
                    // FIXME(xla): Proper error handling.
                    &terrars::SigningKey::from_bytes(&secret_key_bytes).unwrap(),
                    account.account_number,
                    sequence,
                    Coin {
                        denom: "uusd".parse().unwrap(),
                        amount: Decimal256::one(),
                    },
                    // XXX(xla): Based on the observation that txs with two swaps cost around 180k
                    // and the typical gas price in uusd is 0.15.
                    200000,
                    Height::try_from(height.value() + 1).unwrap(),
                    msgs,
                )?;

                match lcd.tx_simulate(&tx).await? {
                    lcd::Response::Ok(tx::SimulateResponse { gas_info, .. }) => {
                        broadcast::observe(
                            Arc::clone(&metrics),
                            messages_len,
                            expected_profit,
                            // TODO(xla): Use actual gas prices.
                            Decimal256::from_uint256(gas_info.gas_used) * Decimal256::percent(15),
                        )
                        .await;

                        if !args.dry_run {
                            let _check = client.broadcast_tx_sync(Transaction::from(tx)).await?;
                            sequence += 1;
                        }
                    },
                    lcd::Response::Error(lcd::Error { message, .. }) => {
                        // TODO(xla): Emit as metric.
                        log::error!("tx simulation failed: {}", message);
                    },
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
        let strategies: Vec<Box<dyn strategy::Strategy<metrics::lcd::Observed<lcd::Http>> + Sync>> =
            vec![Box::new(strategy::metrics::Observed::new(
                Arc::clone(&metrics),
                strategy::multi_dex::SinglePair::default(),
            ))];

        while let Ok(()) = latest_block_receiver.changed().await {
            let latest_block = latest_block_receiver.borrow().clone();
            let height = latest_block.header().height;
            log::debug!(
                "Fetching latest pair prices for {} pools at height {}",
                pools.0.len(),
                height,
            );

            // Ensure latest prices of pairs.
            let start = Instant::now();
            // TODO(xla): Parallelise with batches modulo num cpus.
            // TODO(xla): Query at consistent block height.
            for pool in pools.0.iter_mut() {
                pool.update(&lcd, &mut token_cache).await?;
            }
            log::info!(
                "Updated {} in {:?}",
                pools.0.len(),
                Instant::now().sub(start)
            );

            // Fetch available funds for strategies to incorporate.
            let funds = accounts::fetch(&lcd, &account_id).await?;

            for strategy in &strategies {
                let trades = strategy
                    .execute(latest_block.header.height, &account_id, &funds, &pools)
                    .await;

                for trade in trades {
                    broadcast_sender
                        .send((latest_block.header.height, trade))
                        .await
                        .map_err(|_| Error::BroadcasterGone)?;
                }
            }

            {
                let mekapi = mekapi.clone();
                tokio::spawn(async move {
                    tokio::join!(
                        mekapi.v1_pool_responses::<terrars::dex::astroport::PoolResponse>(
                            Addr::unchecked(terrars::dex::astroport::ADDRESS),
                            height
                        ),
                        mekapi.v1_pool_responses::<terrars::dex::loopmarkets::PoolResponse>(
                            Addr::unchecked(terrars::dex::loopmarkets::ADDRESS),
                            height
                        ),
                        mekapi.v1_pool_responses::<terrars::dex::terraswap::PoolResponse>(
                            Addr::unchecked(terrars::dex::terraswap::ADDRESS),
                            height
                        )
                    )
                });
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
            join = broadcast_handle => join??,
            join = execution_handle => join??,
            complete => break
        }
    }

    client.close().map_err(Error::from)
}
