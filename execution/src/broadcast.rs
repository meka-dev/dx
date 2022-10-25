use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use chainrs::{
    cosmos::{
        auth::BaseAccount,
        bank::MsgSend,
        base::{AccountId, Coin, Denom},
    },
    osmosis::gamm::{MsgSwapExactAmountIn, SwapAmountInRoute},
    proto::cosmos::base::abci::v1beta1::GasInfo,
    tx::{self, Msg},
};
use common::{
    err,
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error, info},
    Context as _,
    Result,
};
use cosmrs::{crypto::secp256k1::SigningKey, Decimal};
use itertools::Itertools;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Unit,
};
use strategy::{Bundle, PoolId, Trade};
use tendermint::{
    abci::{transaction::Hash, Transaction},
    block::Height,
    chain,
};
use tendermint_rpc::{Client as _, WebSocketClient};
use tokio::{
    sync::{mpsc, watch, Mutex},
    time::Instant,
};
use zenith_rs::{AuctionResponse, BidKind, Builder, Http as BuilderHttp};

use crate::tracker::Tracker;

pub type Fee = u64;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Profitable,
    NotProfitable,
}

#[derive(Debug)]
pub struct Simulation {
    fee: u64,
    gas_info: GasInfo,
    outcome: Outcome,
}

pub async fn run<T>(
    mut bundles: mpsc::Receiver<Bundle>,
    auction_receiver: watch::Receiver<AuctionResponse>,
    mut mempool_broadcaster: impl Broadcaster,
    mut auction_broadcaster: impl Broadcaster,
    tx_tracker: Arc<Mutex<T>>,
    execute_trades: Arc<RwLock<bool>>,
) -> Result<()>
where
    T: Tracker,
{
    debug!("Init broadcaster");

    while let Some(bundle) = bundles.recv().await {
        if let Some(addr) = (*tx_tracker.lock().await).is_tracked(&bundle.trade.route) {
            debug!("pool {} in route  ", addr);
            continue;
        }

        let auction;
        {
            auction = auction_receiver.borrow().clone();
        }

        if auction.height != bundle.height {
            if let Ok(simulation) = mempool_broadcaster.simulate(&bundle.trade).await {
                let execute_trades = Arc::clone(&execute_trades);
                let execute_trades =
                    tokio::task::spawn_blocking(move || *execute_trades.read().unwrap())
                        .await
                        .unwrap();

                if execute_trades && simulation.outcome == Outcome::Profitable {
                    if let Ok(resp) = mempool_broadcaster
                        .broadcast(simulation.fee, simulation.gas_info, &bundle, None)
                        .await
                    {
                        let _ = (*tx_tracker.lock().await).track(resp.hash, &bundle.trade.route);
                    }
                }
            }
        } else if let Ok(simulation) = auction_broadcaster.simulate(&bundle.trade).await {
            let execute_trades = Arc::clone(&execute_trades);
            let execute_trades =
                tokio::task::spawn_blocking(move || *execute_trades.read().unwrap())
                    .await
                    .unwrap();

            if execute_trades && simulation.outcome == Outcome::Profitable {
                if let Ok(resp) = auction_broadcaster
                    .broadcast(simulation.fee, simulation.gas_info, &bundle, Some(auction))
                    .await
                {
                    let _ = (*tx_tracker.lock().await).track(resp.hash, &bundle.trade.route);
                }
            }
        };
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum Venue {
    Mempool,
    Auction,
}

#[derive(Debug)]
pub struct BroadcastResponse {
    venue: Venue,
    hash: Hash,
}

#[async_trait]
pub trait Broadcaster {
    async fn broadcast(
        &mut self,
        fee: Fee,
        gas_info: GasInfo,
        bundle: &Bundle,
        auction: Option<AuctionResponse>,
    ) -> Result<BroadcastResponse>;

    async fn simulate(&self, trade: &Trade) -> Result<Simulation>;
}

#[allow(dead_code)]
pub struct Rpc {
    account: Arc<Mutex<BaseAccount>>,
    account_id: AccountId,
    client: WebSocketClient,
    chain_id: chain::Id,
    fee_denom: Denom,
    signing_key_bytes: Vec<u8>,
    treasury: Option<AccountId>,
}

impl Rpc {
    pub fn new(
        client: WebSocketClient,
        chain_id: chain::Id,
        fee_denom: Denom,
        account: Arc<Mutex<BaseAccount>>,
        account_id: AccountId,
        signing_key_bytes: Vec<u8>,
        treasury: Option<AccountId>,
    ) -> Self {
        Self {
            account,
            account_id,
            chain_id,
            fee_denom,
            client,
            signing_key_bytes,
            treasury,
        }
    }
}

fn mk_msg_osmosis(sender: &AccountId, t: &Trade) -> Result<MsgSwapExactAmountIn> {
    let min_out = t.size_to_int();
    let path = t
        .legs
        .iter()
        .map(|leg| SwapAmountInRoute {
            pool_id: leg.pool.id.clone(),
            token_out_denom: leg.out_token.id.clone(),
        })
        .collect_vec();
    Ok(MsgSwapExactAmountIn {
        sender: sender.to_string(),
        routes: path,
        token_in: Some(Coin {
            denom: chainrs::cosmos::base::Denom::from_str(&t.in_token.id)?,
            amount: t
                .size_to_int()
                .0
                .as_u64() // XXX
                .into(),
        }),
        token_out_min_amount: min_out.to_string(),
    })
}

fn mk_payments(
    from: &AccountId,
    auction: &AuctionResponse,
    bid_amount: u64,
) -> Result<Vec<MsgSend>> {
    auction
        .payments
        .iter()
        .map(|e| {
            Ok(MsgSend {
                from_address: from.to_string(),
                to_address: e.address.clone(),
                amount: vec![Coin {
                    denom: chainrs::cosmos::base::Denom::from_str(&e.denom)?,
                    amount: Decimal::from((bid_amount as f64 * e.allocation) as u64),
                }],
            })
        })
        .collect::<Result<Vec<_>>>()
}

#[async_trait]
impl Broadcaster for Rpc {
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        bundle: &Bundle,
        _auction: Option<AuctionResponse>,
    ) -> Result<BroadcastResponse> {
        let msg = mk_msg_osmosis(&self.account_id, &bundle.trade)?.to_any()?;

        let account;
        {
            account = self.account.lock().await.clone();
        }
        let tx = tx::sign(
            self.chain_id.clone(),
            &SigningKey::from_bytes(&self.signing_key_bytes)
                .map_err(|err| err!("failed to craft signing key from bytes: {}", err))?,
            account.account_number,
            account.sequence,
            // NB(xla): Using fixed gas prices here, could diverge.
            fee,
            "uosmo".to_string(),
            (gas_info.gas_used as f64 * 1.10) as u64,
            None,
            None,
            Height::try_from(0_u64).unwrap(),
            vec![msg],
        )
        .context("failed to sign transaction")?;

        let check = self
            .client
            .broadcast_tx_sync(Transaction::from(tx))
            .await
            .context("failed to broadcast tx")?;
        // Advance sequence if tx succeeded.
        if check.code.is_ok() {
            let mut account = self.account.lock().await;
            account.sequence += 1;
            Ok(BroadcastResponse {
                venue: Venue::Mempool,
                hash: check.hash,
            })
        } else {
            Err(err!("{:}", check.log))
        }
    }

    async fn simulate(&self, trade: &Trade) -> Result<Simulation> {
        let expected_profit: u128 = trade.expected_profit_to_int().into();
        let gas_used = trade
            .route
            .iter()
            .fold(0u64, |acc, m| acc + gas_estimator(m));

        let fee = (gas_used as f64 * 0.007f64) as u64;
        let outcome = if expected_profit > fee.into() {
            Outcome::Profitable
        } else {
            Outcome::NotProfitable
        };
        Ok(Simulation {
            fee,
            gas_info: GasInfo {
                gas_wanted: 0,
                gas_used,
            },
            outcome,
        })
    }
}

// NB(pm): needs rework
fn gas_estimator(_addr: &PoolId) -> u64 {
    85000
}

pub struct BuilderRpc<B: Builder> {
    builder: B,
    account: Arc<Mutex<BaseAccount>>,
    account_id: AccountId,
    chain_id: chain::Id,
    fee_denom: Denom,
    signing_key_bytes: Vec<u8>,
}

impl BuilderRpc<BuilderHttp> {
    pub fn new(
        builder: BuilderHttp,
        chain_id: chain::Id,
        fee_denom: Denom,
        account: Arc<Mutex<BaseAccount>>,
        account_id: AccountId,
        signing_key_bytes: Vec<u8>,
    ) -> Self {
        Self {
            builder,
            account,
            account_id,
            chain_id,
            fee_denom,
            signing_key_bytes,
        }
    }
}

#[async_trait]
impl Broadcaster for BuilderRpc<BuilderHttp> {
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        bundle: &Bundle,
        auction: Option<AuctionResponse>,
    ) -> Result<BroadcastResponse> {
        let arb = mk_msg_osmosis(&self.account_id, &bundle.trade)?.to_any()?;
        // XXX: hardcoded 10% bid
        let bid_amount =
            ((bundle.trade.expected_profit_to_int().0.as_u64() - fee) as f64 * 0.1) as u64;
        let bids = mk_payments(
            &self.account_id,
            &auction.ok_or(err!("no auction provided for zenith broadcast"))?,
            bid_amount,
        )?
        .into_iter()
        .map(|m| m.to_any())
        .collect::<Result<Vec<_>>>()?;
        let account;
        {
            account = self.account.lock().await.clone();
        }
        let mut msgs = vec![arb];
        msgs.extend(bids);
        let tx = tx::sign(
            self.chain_id.clone(),
            &SigningKey::from_bytes(&self.signing_key_bytes)
                .map_err(|err| err!("failed to craft signing key from bytes: {}", err))?,
            account.account_number,
            account.sequence,
            // NB(xla): Using fixed gas prices here, could diverge.
            fee,
            self.fee_denom.to_string(),
            (gas_info.gas_used as f64 * 1.10) as u64,
            None,
            None,
            Height::try_from(0_u64).unwrap(),
            msgs,
        )
        .context("failed to sign transaction")?;

        let mut txs = bundle.txs.clone();
        txs.push(tx);
        let r = self
            .builder
            .bid(self.chain_id.to_string(), bundle.height, BidKind::Top, txs)
            .await?;

        Ok(BroadcastResponse {
            venue: Venue::Auction,
            hash: r
                .tx_hashes
                .first()
                .ok_or(err!("did not receive hashes from zenith"))?
                .parse()?,
        })
    }

    async fn simulate(&self, trade: &Trade) -> Result<Simulation> {
        let expected_profit: u128 = trade.expected_profit_to_int().into();
        let gas_used = trade
            .route
            .iter()
            .fold(0u64, |acc, m| acc + gas_estimator(m))
            + 2 * 30000; // 2x bank transfer

        let fee = (gas_used as f64 * 0.007f64) as u64;
        let outcome = if expected_profit > fee.into() {
            Outcome::Profitable
        } else {
            Outcome::NotProfitable
        };
        Ok(Simulation {
            fee,
            gas_info: GasInfo {
                gas_wanted: 0,
                gas_used,
            },
            outcome,
        })
    }
}

impl Labeled for Rpc {
    const NAME: Name = Name::Rpc;
}

impl Labeled for BuilderRpc<BuilderHttp> {
    const NAME: Name = Name::Auction;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    Rpc,
    Auction,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    Broadcast,
    Simulate,
}

pub trait Labeled {
    const NAME: Name;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
struct CallLabels {
    name: Name,
    method: Method,
    result: ResultLabel,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
struct PerformanceLabels {
    name: Name,
    method: Method,
}

pub struct Observed<N> {
    next: N,

    call_duration: Family<CallLabels, Histogram>,
    estimated_fee_uusd: Family<PerformanceLabels, Counter>,
    expected_profit_after_fee_uusd: Family<PerformanceLabels, Counter>,
}

impl<N> Observed<N> {
    pub fn new(registry: &mut Registry, next: N) -> Self {
        let call_duration = Family::<CallLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "broadcast_call_duration",
            "Broacast call durations",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );
        let estimated_fee_uusd = Family::<PerformanceLabels, Counter>::default();
        registry.register(
            "broadast_estimated_fee_uusd",
            "Estimated transaction fee",
            Box::new(estimated_fee_uusd.clone()),
        );
        let expected_profit_after_fee_uusd = Family::<PerformanceLabels, Counter>::default();
        registry.register(
            "broadcast_expected_profit_after_fee_uusd",
            "Expected profit after fees have been subtracted",
            Box::new(expected_profit_after_fee_uusd.clone()),
        );

        Self {
            next,

            call_duration,
            estimated_fee_uusd,
            expected_profit_after_fee_uusd,
        }
    }
}

impl<N> Observed<N>
where
    N: Labeled,
{
    fn observe(&self, method: Method, start: Instant, is_err: bool) {
        self.call_duration
            .get_or_create(&CallLabels {
                name: N::NAME,
                method,
                result: if is_err {
                    ResultLabel::Error
                } else {
                    ResultLabel::Success
                },
            })
            .observe(start.elapsed().as_secs_f64());
    }
}

// XXX: track bids
#[async_trait]
impl<N> Broadcaster for Observed<N>
where
    N: Broadcaster + Labeled + Send + Sync + 'static,
{
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        bundle: &Bundle,
        auction: Option<AuctionResponse>,
    ) -> Result<BroadcastResponse> {
        let expected_profit = bundle.trade.expected_profit_to_int().0.as_u64();
        let method = Method::Broadcast;
        let start = Instant::now();
        let res = self.next.broadcast(fee, gas_info, bundle, auction).await;

        self.observe(method.clone(), start, res.is_err());

        let perf_labels = PerformanceLabels {
            name: N::NAME,
            method,
        };
        self.estimated_fee_uusd
            .get_or_create(&perf_labels)
            .inc_by(fee);

        if fee < expected_profit {
            self.expected_profit_after_fee_uusd
                .get_or_create(&perf_labels)
                .inc_by(expected_profit - fee);
        }

        match res {
            Ok(check) => {
                info!(
                    "msg=submit height={} hash={} venue={:?}",
                    bundle.height, check.hash, check.venue,
                );

                Ok(check)
            },
            Err(err) => {
                error!("failed to submit tx: {}", err);
                Err(err)
            },
        }
    }

    async fn simulate(&self, trade: &Trade) -> Result<Simulation> {
        let start = Instant::now();
        let res = self.next.simulate(trade).await;

        self.observe(Method::Simulate, start, res.is_err());

        match res {
            Ok(simulation) => {
                let profit = if trade.expected_profit_to_int().0.as_u64() > simulation.fee {
                    trade.expected_profit_to_int().0.as_u64() - simulation.fee
                } else {
                    0
                };
                info!(
                    "legs={} expected_profit={} estimated_fee={} delta={} outcome={:?}",
                    trade.legs.len(),
                    trade.expected_profit,
                    simulation.fee,
                    profit,
                    simulation.outcome,
                );

                Ok(simulation)
            },
            Err(err) => {
                error!("failed to simulate tx: {}", err);
                Err(err)
            },
        }
    }
}
