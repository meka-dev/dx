use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;
use chainrs::{
    cosmos::{auth::BaseAccount, base::AccountId},
    proto::cosmos::base::abci::v1beta1::GasInfo,
    tx,
};
use common::{
    err,
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error, info},
    Context as _,
    Result,
};
use cosmrs::{crypto::secp256k1::SigningKey, tx::Msg};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Unit,
};
use strategy::Trade;
use tendermint::{abci::Transaction, block::Height, chain};
use tendermint_rpc::{endpoint::broadcast, Client as _, WebSocketClient};
use tokio::{
    sync::{mpsc, Mutex},
    time::Instant,
};

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

pub async fn run<B, T>(
    mut trades: mpsc::Receiver<Trade>,
    mut broadcaster: B,
    tx_tracker: Arc<Mutex<T>>,
    execute_trades: Arc<RwLock<bool>>,
) -> Result<()>
where
    B: Broadcaster,
    T: Tracker,
{
    debug!("Init broadcaster");

    while let Some(trade) = trades.recv().await {
        if let Some(addr) = (*tx_tracker.lock().await).is_tracked(&trade.route) {
            debug!("pool {} in route  ", addr);
            continue;
        }

        if let Ok(simulation) = broadcaster.simulate(&trade).await {
            let execute_trades = Arc::clone(&execute_trades);
            let execute_trades =
                tokio::task::spawn_blocking(move || *execute_trades.read().unwrap())
                    .await
                    .unwrap();

            if execute_trades && simulation.outcome == Outcome::Profitable {
                if let Ok(check) = broadcaster
                    .broadcast(simulation.fee, simulation.gas_info, &trade)
                    .await
                {
                    if check.code.is_ok() {
                        let _ = (*tx_tracker.lock().await).track(check.hash, &trade.route);
                    }
                }
            }
        }
    }

    Ok(())
}

#[async_trait]
pub trait Broadcaster {
    async fn broadcast(
        &mut self,
        fee: Fee,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response>;

    async fn simulate(&self, trade: &Trade) -> Result<Simulation>;
}

pub struct Rpc {
    account: BaseAccount,
    client: WebSocketClient,
    chain_id: chain::Id,
    signing_key_bytes: Vec<u8>,
    sequence: u64,
    treasury: Option<AccountId>,
}

impl Rpc {
    pub fn new(
        client: WebSocketClient,
        chain_id: chain::Id,
        account: BaseAccount,
        signing_key_bytes: Vec<u8>,
        treasury: Option<AccountId>,
    ) -> Self {
        let sequence = account.sequence;

        Self {
            account,
            chain_id,
            client,
            signing_key_bytes,
            sequence,
            treasury,
        }
    }
}

#[async_trait]
impl Broadcaster for Rpc {
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response> {
        let messages = trade
            .messages
            .clone()
            .into_iter()
            .map(|msg| msg.to_any().unwrap())
            .collect::<Vec<_>>();
        let tx = tx::sign(
            self.chain_id.clone(),
            &SigningKey::from_bytes(&self.signing_key_bytes)
                .map_err(|err| err!("failed to craft signing key from bytes: {}", err))?,
            self.account.account_number,
            self.sequence,
            // NB(xla): Using fixed gas prices here, could diverge.
            fee,
            (gas_info.gas_used as f64 * 1.10) as u64,
            None,
            self.treasury.clone(),
            Height::try_from(0_u64).unwrap(),
            messages,
        )
        .context("failed to sign transaction")?;

        let check = self
            .client
            .broadcast_tx_sync(Transaction::from(tx))
            .await
            .context("failed to broadcast tx")?;
        // Advance sequence if tx succeeded.
        if check.code.is_ok() {
            self.sequence += 1;
        }

        Ok(check)
    }

    async fn simulate(&self, trade: &Trade) -> Result<Simulation> {
        let expected_profit = trade.expected_profit.0.as_u64();
        let gas_used = trade
            .route
            .iter()
            .fold(0u64, |acc, m| acc + gas_estimator(m))
            + 200000;
        // NB: add a static overhead for calling via proxy

        let fee = (gas_used as f64 * 0.18f64) as u64;
        let outcome = if expected_profit > fee {
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
fn gas_estimator(addr: &AccountId) -> u64 {
    let expensive = HashSet::from([
        "terra1qpd9n7afwf45rkjlpujrrdfh83pldec8rpujgn".to_owned(), // bLuna/ust
        "terra18r6rdnkgrg74zew3d8l9nhk0m4xanpeukw3e20".to_owned(), // bLuna/ust
        "terra1jxazgm67et0ce260kvrpfv50acuushpjsz2y0p".to_owned(), // bLuna/luna
        "terra1v93ll6kqp33unukuwls3pslquehnazudu653au".to_owned(), // bLuna/luna
        "terra1x8h5gan6vey5cz2xfyst74mtqsj7746fqj2hze".to_owned(), // bLuna/lunax
        "terra10lv5wz84kpwxys7jeqkfxx299drs3vnw0lj8mz".to_owned(), // nLuna/psi
        "terra1zvn8z6y8u2ndwvsjhtpsjsghk6pa6ugwzxp6vx".to_owned(), // nLuna/psi
        "terra1c5swgtnuunpf75klq5uztynurazuwqf0mmmcyy".to_owned(), // beth/meth
        "terra1myl709y74vrdcyuxy6g9wv5l2sgah4e9lstnwe".to_owned(), // beth/ust
    ]);

    if expensive.contains(&addr.to_string()) {
        625000
    } else {
        350000
    }
}

impl Labeled for Rpc {
    const NAME: Name = Name::Rpc;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    Rpc,
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

#[async_trait]
impl<N> Broadcaster for Observed<N>
where
    N: Broadcaster + Labeled + Send + Sync + 'static,
{
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response> {
        let expected_profit = trade.expected_profit.0.as_u64();
        let method = Method::Broadcast;
        let start = Instant::now();
        let res = self.next.broadcast(fee, gas_info, trade).await;

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
                    "msg=submit hash={} code={:?} log={}",
                    check.hash, check.code, check.log
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
                info!(
                    "messages={} expected_profit={} estimated_fee={} outcome={:?}",
                    trade.messages.len(),
                    trade.expected_profit,
                    simulation.fee,
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
