use std::{fmt::Display, sync::Arc};

use prometheus_client::encoding::text::Encode;
use tendermint::block;
use tokio::{sync::Mutex, time::Instant};

use crate::{accounts, config, metrics, strategy::Strategy};

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
pub struct EmitLabels {
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
    ) -> Vec<Vec<terrars::MsgExecuteContract>> {
        let start = Instant::now();
        let txs = self.next.execute(height, sender, funds, pools).await;

        let elapsed = start.elapsed().as_secs_f64();
        let output = match txs.len() {
            0 => Output::NoTrade,
            _ => Output::FoundTrades,
        };

        log::debug!(
            "version={} module={} name={} height={} output={} txs={} duration={}",
            self.version,
            self.next.module(),
            self.next.name(),
            height,
            output.clone(),
            txs.len(),
            elapsed,
        );

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
            .strategy_tx_emit
            .get_or_create(&EmitLabels {
                version: config::VERSION.to_string(),
                module: self.next.module(),
                name: self.next.name(),
            })
            .inc_by(txs.len().try_into().unwrap());
        txs.iter().for_each(|msgs| {
            metrics
                .strategy_msgs_emit
                .get_or_create(&EmitLabels {
                    version: config::VERSION.to_string(),
                    module: self.next.module(),
                    name: self.next.name(),
                })
                .inc_by(msgs.len().try_into().unwrap());
        });

        txs
    }
}
