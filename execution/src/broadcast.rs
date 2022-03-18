use std::sync::{Arc, RwLock};

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use cosmwasm_bignumber::Decimal256;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Unit,
};
use tendermint::{abci::Transaction, block::Height, chain};
use tendermint_rpc::{endpoint::broadcast, Client as _, WebSocketClient};
use tokio::{
    sync::{mpsc, Mutex},
    time::Instant,
};

use common::{
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error, info},
};
use terrars::{base::abci::GasInfo, lcd, tx, BaseAccount, Msg as _, SigningKey};

use crate::{strategy::Trade, tracker::Tracker};

pub type Fee = u64;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Profitable,
    NotProfitable,
}

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
            let execute_trades = *execute_trades
                .read()
                .expect("failed to acquire poisoned lock");

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

pub struct Rpc<L> {
    account: BaseAccount,
    client: WebSocketClient,
    chain_id: chain::Id,
    lcd: L,
    signing_key_bytes: Vec<u8>,
    sequence: u64,
}

impl<L> Rpc<L> {
    pub fn new(
        client: WebSocketClient,
        lcd: L,
        chain_id: chain::Id,
        account: BaseAccount,
        signing_key_bytes: Vec<u8>,
    ) -> Self {
        let sequence = account.sequence;

        Self {
            account,
            chain_id,
            client,
            lcd,
            signing_key_bytes,
            sequence,
        }
    }
}

#[async_trait]
impl<L> Broadcaster for Rpc<L>
where
    L: lcd::Lcd,
{
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
        let tx = terrars::tx::sign(
            self.chain_id.clone(),
            &SigningKey::from_bytes(&self.signing_key_bytes).map_err(|err| {
                anyhow::anyhow!("failed to craft signing key from bytes: {}", err)
            })?,
            self.account.account_number,
            self.sequence,
            // TODO(xla): Use actual gas prices.
            fee,
            (gas_info.gas_used as f64 * 1.10) as u64,
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
        let expected_profit = convert_expected_profit(trade.expected_profit);
        let messages = trade
            .messages
            .clone()
            .into_iter()
            .map(|msg| msg.to_any().unwrap())
            .collect::<Vec<_>>();
        let simulate_tx = tx::sign(
            self.chain_id.clone(),
            &SigningKey::from_bytes(&self.signing_key_bytes).map_err(|err| {
                anyhow::anyhow!("failed to craft signing key from bytes: {}", err)
            })?,
            self.account.account_number,
            self.sequence,
            0,
            0,
            Height::try_from(0_u64).unwrap(),
            messages,
        )?;

        match self.lcd.tx_simulate(&simulate_tx).await? {
            lcd::Response::Ok(tx::SimulateResponse { gas_info, .. }) => {
                let fee = (gas_info.gas_used as f64 * 0.18) as u64;

                let outcome = if expected_profit > fee {
                    Outcome::Profitable
                } else {
                    Outcome::NotProfitable
                };

                Ok(Simulation {
                    fee,
                    gas_info,
                    outcome,
                })
            },
            lcd::Response::Error(lcd::Error { message, .. }) => {
                Err(anyhow::anyhow!("tx simulation failed: {}", message))
            },
        }
    }
}

impl<L> Labeled for Rpc<L> {
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
        let expected_profit = convert_expected_profit(trade.expected_profit);
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
                    convert_expected_profit(trade.expected_profit),
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

fn convert_expected_profit(expected: Decimal256) -> u64 {
    (expected.0 / 10u64.pow(12).into()).as_u64()
}
