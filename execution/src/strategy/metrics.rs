use std::{fmt::Display, sync::Arc};

use cosmwasm_bignumber::{Decimal256, Uint256};
use prometheus_client::encoding::text::Encode;
use tendermint::block;
use tokio::{sync::Mutex, time::Instant};

use crate::{accounts, config, metrics, strategy::Strategy};

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

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub enum Output {
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
pub struct DurationLabels {
    version: String,
    module: Module,
    name: Name,
    output: Output,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct TradesLabel {
    version: String,
    module: Module,
    name: Name,
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

pub struct Observed<S> {
    next: S,
    metrics: Arc<Mutex<metrics::Metrics>>,
    version: String,
}

impl<S> Observed<S> {
    pub fn new(version: impl Into<String>, metrics: Arc<Mutex<metrics::Metrics>>, next: S) -> Self {
        Self {
            metrics,
            next,
            version: version.into(),
        }
    }
}

#[async_trait::async_trait]
impl<S, L> Strategy<L> for Observed<S>
where
    S: Strategy<L> + Labeled + Sync,
{
    async fn execute(
        &self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &crate::dex::Pools<L>,
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
            "version={} module={} name={} height={} output={} size={} expected_profit={} trades={} duration={}",
            self.version,
            self.next.module(),
            self.next.name(),
            height,
            output.clone(),
            size_total,
            expected_profit_total,
            trades.len(),
            elapsed,
        );

        let trade_labels = TradesLabel {
            version: config::VERSION.to_string(),
            module: self.next.module(),
            name: self.next.name(),
        };
        let metrics = self.metrics.lock().await;
        metrics
            .strategy_duration
            .get_or_create(&DurationLabels {
                version: config::VERSION.to_string(),
                module: self.next.module(),
                name: self.next.name(),
                output: output.clone(),
            })
            .observe(elapsed);
        metrics
            .strategy_size_max
            .get_or_create(&trade_labels)
            .set((size_max.0 / 10u64.pow(12).into()).as_u64());
        metrics
            .strategy_expected_profit_max
            .get_or_create(&trade_labels)
            .set((expected_profit_max.0 / 10u64.pow(12).into()).as_u64());

        trades.iter().for_each(
            |Trade {
                 size,
                 expected_profit,
                 messages,
             }| {
                metrics.strategy_trades.get_or_create(&trade_labels).inc();
                metrics
                    .strategy_trade_size_uusd
                    .get_or_create(&trade_labels)
                    .inc_by((size.0 / 10u64.pow(12).into()).as_u64());
                metrics
                    .strategy_trade_expected_profit_uusd
                    .get_or_create(&trade_labels)
                    .inc_by((expected_profit.0 / 10u64.pow(12).into()).as_u64());
                metrics
                    .strategy_trade_messages_emitted
                    .get_or_create(&trade_labels)
                    .inc_by(messages.len().try_into().unwrap());
            },
        );

        trades
    }
}
