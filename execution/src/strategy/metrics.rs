use std::sync::Arc;

use prometheus_client::encoding::text::Encode;
use tendermint::block;
use tokio::{sync::Mutex, time::Instant};

use crate::{accounts, config, metrics, strategy::Strategy};

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub enum Module {
    MultiDex,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    SinglePair,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub enum Output {
    NoTrade,
    FoundTrades,
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

pub struct Instrumented<S>
where
    S: Strategy,
{
    next: S,
    metrics: Arc<Mutex<metrics::Metrics>>,
}

impl<S> Instrumented<S>
where
    S: Strategy,
{
    pub fn new(metrics: Arc<Mutex<metrics::Metrics>>, next: S) -> Self {
        Self { metrics, next }
    }
}

#[async_trait::async_trait]
impl<S> Strategy for Instrumented<S>
where
    S: Strategy + Labeled + Sync,
{
    async fn execute(
        &self,
        height: block::Height,
        account_id: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &crate::dex::Pools,
    ) -> Vec<Vec<terrars::Message>> {
        let start = Instant::now();
        let txs = self.next.execute(height, account_id, funds, pools).await;

        let metrics = self.metrics.lock().await;
        metrics
            .strategy_duration
            .get_or_create(&DurationLabels {
                version: config::VERSION.to_string(),
                module: self.next.module(),
                name: self.next.name(),
                output: match txs.len() {
                    0 => Output::NoTrade,
                    _ => Output::FoundTrades,
                },
            })
            .observe(start.elapsed().as_secs_f64());
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
