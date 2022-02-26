use std::{sync::Arc, time::Instant};

use anyhow::Result;
use cosmwasm_std::Addr;
use prometheus_client::encoding::text::Encode;
use tendermint::block;
use terrars::mekapi::Mekapi;
use tokio::sync::Mutex;

use terrars::dex;

use crate::metrics;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    V1PoolResponses,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    factory: String,
    method: Method,
    result: metrics::ResultLabel,
}

#[derive(Clone)]
pub struct Observed<L> {
    next: L,
    metrics: Arc<Mutex<metrics::Metrics>>,
}

impl<L> Observed<L> {
    pub fn new(metrics: Arc<Mutex<metrics::Metrics>>, next: L) -> Self {
        Self { metrics, next }
    }
}

#[async_trait::async_trait]
impl<L> Mekapi for Observed<L>
where
    L: Mekapi,
{
    async fn v1_pool_responses(
        &self,
        factory: Addr,
        height: block::Height,
    ) -> Result<Vec<dex::Pool>> {
        let start = Instant::now();
        let res = self.next.v1_pool_responses(factory.clone(), height).await;
        let elapsed = start.elapsed();

        self.metrics
            .lock()
            .await
            .mekapi_call_duration
            .get_or_create(&Labels {
                factory: factory.to_string(),
                method: Method::V1PoolResponses,
                result: if res.is_err() {
                    metrics::ResultLabel::Error
                } else {
                    metrics::ResultLabel::Success
                },
            })
            .observe(elapsed.as_secs_f64());

        log::debug!(
            "method=v1_pool_resposes contract={} height={} elapsed={:?} err={:?}",
            factory.to_string(),
            height.to_string(),
            elapsed,
            res.as_ref().err()
        );

        res
    }
}
