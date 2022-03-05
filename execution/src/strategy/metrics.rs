use std::fmt::Display;

use cosmwasm_bignumber::{Decimal256, Uint256};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, gauge::Gauge, histogram::Histogram},
    registry::Unit,
};
use tendermint::block;
use tokio::time::Instant;

use common::metrics::{Registry, BUCKETS};
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
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SinglePair => write!(f, "SinglePair"),
        }
    }
}

pub trait Labeled {
    const MODULE: Module;
    const NAME: Name;

    fn module(&self) -> Module {
        Self::MODULE
    }

    fn name(&self) -> Name {
        Self::NAME
    }
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
    output: Output,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct TradesLabel {
    module: Module,
    name: Name,
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

impl<S> Observed<S> {
    pub fn new(registry: &mut Registry, next: S) -> Self {
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
}

#[async_trait::async_trait]
impl<S> Strategy for Observed<S>
where
    S: Strategy + Labeled + Sync,
{
    async fn execute(
        &self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools,
    ) -> Vec<Trade> {
        let start = Instant::now();
        let trades = self.next.execute(height, sender, funds, pools).await;

        let elapsed = start.elapsed().as_secs_f64();
        let (size_total, size_max) = trades.iter().fold(
            (
                Decimal256::from_uint256(Uint256::zero()),
                Decimal256::from_uint256(Uint256::zero()),
            ),
            |(total, max), Trade { size, .. }| {
                let max = if *size > max { *size } else { max };
                (total + *size, max)
            },
        );
        let (expected_profit_total, expected_profit_max) = trades.iter().fold(
            (
                Decimal256::from_uint256(Uint256::zero()),
                Decimal256::from_uint256(Uint256::zero()),
            ),
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

        log::debug!(
            "module={} name={} height={} output={} size={} expected_profit={} trades={} duration={}",
            self.next.module(),
            self.next.name(),
            height,
            output,
            size_total,
            expected_profit_total,
            trades.len(),
            elapsed,
        );

        let trade_labels = TradesLabel {
            module: self.next.module(),
            name: self.next.name(),
        };
        self.execute_duration
            .get_or_create(&DurationLabels {
                module: self.next.module(),
                name: self.next.name(),
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

        trades
    }
}
