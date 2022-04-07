use std::fmt::Display;

use cosmwasm_bignumber::Uint256;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, gauge::Gauge, histogram::Histogram},
    registry::Unit,
};
use tendermint::block;
use tokio::time::Instant;

use common::{
    metrics::{Registry, BUCKETS},
    tracing::debug,
};
use terrars::dex;

use crate::{accounts, strategy::Strategy};

use super::Trade;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Module {
    MultiDex,
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::MultiDex => write!(f, "MultiDex"),
        }
    }
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    SinglePair,
    MultiHop,
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SinglePair => write!(f, "SinglePair"),
            Self::MultiHop => write!(f, "MultiHop"),
        }
    }
}

pub trait Labeled {
    const MODULE: Module;
    const NAME: Name;
}

#[derive(Clone, Copy, Debug, Hash, Encode, Eq, PartialEq)]
enum Method {
    ExecuteMempoolTx,
    ExecutePools,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
enum Output {
    NoTrade,
    FoundTrades,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NoTrade => write!(f, "NoTrade"),
            Self::FoundTrades => write!(f, "FoundTrades"),
        }
    }
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct DurationLabels {
    module: Module,
    name: Name,
    method: Method,
    output: Output,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct TradesLabel {
    module: Module,
    name: Name,
    method: Method,
}

pub struct Observed<S> {
    execute_duration: Family<DurationLabels, Histogram>,
    expected_profit_max: Family<TradesLabel, Gauge>,
    size_max: Family<TradesLabel, Gauge>,
    trades: Family<TradesLabel, Counter>,
    trade_expected_profit_uusd: Family<TradesLabel, Counter>,
    trade_messages_emitted: Family<TradesLabel, Counter>,
    trade_size_uusd: Family<TradesLabel, Counter>,

    next: S,
}

impl<N> Observed<N>
where
    N: Labeled,
{
    pub fn new(registry: &mut Registry, next: N) -> Self {
        let execute_duration = Family::<DurationLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "strategy_execute_duration",
            "Strategy execution durations",
            Unit::Seconds,
            Box::new(execute_duration.clone()),
        );
        let size_max = Family::<TradesLabel, Gauge>::default();
        registry.register(
            "strategy_size_max",
            "Largest trade size within a strategy execution",
            Box::new(size_max.clone()),
        );
        let expected_profit_max = Family::<TradesLabel, Gauge>::default();
        registry.register(
            "strategy_expected_profit_max",
            "Largest trade profit within a strategy execution",
            Box::new(expected_profit_max.clone()),
        );
        let trades = Family::<TradesLabel, Counter>::default();
        registry.register(
            "strategy_trades",
            "Number of trades found by strategy execution",
            Box::new(trades.clone()),
        );
        let trade_expected_profit_uusd = Family::<TradesLabel, Counter>::default();
        registry.register(
            "strategy_trade_expected_profit_uusd",
            "Profit expected to be made from trades",
            Box::new(trade_expected_profit_uusd.clone()),
        );
        let trade_messages_emitted = Family::<TradesLabel, Counter>::default();
        registry.register(
            "strategy_trade_messages_emitted",
            "Number of messages emitted by strategies",
            Box::new(trade_messages_emitted.clone()),
        );
        let trade_size_uusd = Family::<TradesLabel, Counter>::default();
        registry.register(
            "strategy_trade_size_uusd",
            "Liquidity required to execute optimal strategy trades",
            Box::new(trade_size_uusd.clone()),
        );

        Self {
            execute_duration,
            expected_profit_max,
            size_max,
            trades,
            trade_expected_profit_uusd,
            trade_messages_emitted,
            trade_size_uusd,

            next,
        }
    }

    fn observe(&self, method: Method, start: Instant, height: block::Height, trades: &[Trade]) {
        let elapsed = start.elapsed().as_secs_f64();
        let (size_total, size_max) = trades.iter().fold(
            (Uint256::zero(), Uint256::zero()),
            |(total, max), Trade { size, .. }| {
                let max = if *size > max { *size } else { max };
                (total + *size, max)
            },
        );
        let (expected_profit_total, expected_profit_max) = trades.iter().fold(
            (Uint256::zero(), Uint256::zero()),
            |(total, max),
             Trade {
                 expected_profit, ..
             }| {
                let max = if *expected_profit > max {
                    *expected_profit
                } else {
                    max
                };
                (total + *expected_profit, max)
            },
        );
        let output = match trades.len() {
            0 => Output::NoTrade,
            _ => Output::FoundTrades,
        };

        debug!(
            "module={} name={} method={:?} height={} output={} size={} expected_profit={} trades={} duration={}",
            N::MODULE,
            N::NAME,
            method,
            height,
            output,
            size_total,
            expected_profit_total,
            trades.len(),
            elapsed,
        );

        let trade_labels = TradesLabel {
            module: N::MODULE,
            name: N::NAME,
            method,
        };
        self.execute_duration
            .get_or_create(&DurationLabels {
                module: N::MODULE,
                name: N::NAME,
                method,
                output,
            })
            .observe(elapsed);
        self.size_max
            .get_or_create(&trade_labels)
            .set((size_max.0 / 10u64.pow(12).into()).as_u64());
        self.expected_profit_max
            .get_or_create(&trade_labels)
            .set((expected_profit_max.0 / 10u64.pow(12).into()).as_u64());

        trades.iter().for_each(
            |Trade {
                 size,
                 expected_profit,
                 messages,
                 ..
             }| {
                self.trades.get_or_create(&trade_labels).inc();
                self.trade_size_uusd
                    .get_or_create(&trade_labels)
                    .inc_by((size.0 / 10u64.pow(12).into()).as_u64());
                self.trade_expected_profit_uusd
                    .get_or_create(&trade_labels)
                    .inc_by((expected_profit.0 / 10u64.pow(12).into()).as_u64());
                self.trade_messages_emitted
                    .get_or_create(&trade_labels)
                    .inc_by(messages.len().try_into().unwrap());
            },
        );
    }
}

#[async_trait::async_trait]
impl<S> Strategy for Observed<S>
where
    S: Strategy + Labeled + Sync,
{
    async fn execute_mempool_tx(
        &mut self,
        height: block::Height,
        sender: &terrars::AccountId,
        tx: &terrars::Tx,
    ) -> Vec<Trade> {
        let start = Instant::now();
        let trades = self.next.execute_mempool_tx(height, sender, tx).await;

        self.observe(Method::ExecuteMempoolTx, start, height, &trades);

        trades
    }

    async fn execute_pools(
        &mut self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools,
    ) -> Vec<Trade> {
        let start = Instant::now();
        let trades = self.next.execute_pools(height, sender, funds, pools).await;

        self.observe(Method::ExecutePools, start, height, &trades);

        trades
    }
}
