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
pub enum Coin {
    Uusd,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct DurationLabels {
    version: String,
    module: Module,
    name: Name,
    output: Output,
}
#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct CoinLabels {
    version: String,
    module: Module,
    name: Name,
    coin: Coin,
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

pub struct Observed<S>
where
    S: Strategy,
{
    next: S,
    metrics: Arc<Mutex<metrics::Metrics>>,
    version: String,
}

impl<S> Observed<S>
where
    S: Strategy,
{
    pub fn new(version: impl Into<String>, metrics: Arc<Mutex<metrics::Metrics>>, next: S) -> Self {
        Self {
            metrics,
            next,
            version: version.into(),
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
        pools: &crate::dex::Pools,
    ) -> Vec<Trade> {
        let start = Instant::now();
        let trades = self.next.execute(height, sender, funds, pools).await;

        let elapsed = start.elapsed().as_secs_f64();
        let required_liquidity = trades.iter().fold(
            Decimal256::from_uint256(Uint256::zero()),
            |acc, (liq, _, _)| acc + *liq,
        );
        let expected_profit = trades.iter().fold(
            Decimal256::from_uint256(Uint256::zero()),
            |acc, (_, profit, _)| acc + *profit,
        );
        let output = match trades.len() {
            0 => Output::NoTrade,
            _ => Output::FoundTrades,
        };

        log::debug!(
            "version={} module={} name={} height={} output={} required_liquidity={} expected_profit={} trades={} duration={}",
            self.version,
            self.next.module(),
            self.next.name(),
            height,
            output.clone(),
            required_liquidity,
            expected_profit,
            trades.len(),
            elapsed,
        );

        let coin_labels = CoinLabels {
            version: config::VERSION.to_string(),
            module: self.next.module(),
            name: self.next.name(),
            coin: Coin::Uusd,
        };
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
            .strategy_required_liquidity
            .get_or_create(&coin_labels)
            .inc_by(required_liquidity.0.as_u64());
        metrics
            .strategy_expected_profit
            .get_or_create(&coin_labels)
            .inc_by(expected_profit.0.as_u64());
        metrics
            .strategy_trades_found
            .get_or_create(&trade_labels)
            .inc_by(trades.len().try_into().unwrap());
        trades.iter().for_each(|(_, _, msgs)| {
            metrics
                .strategy_msgs_emit
                .get_or_create(&trade_labels)
                .inc_by(msgs.len().try_into().unwrap());
        });

        trades
    }
}
