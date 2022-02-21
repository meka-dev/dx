use std::{sync::Arc, time::Instant};

use anyhow::Result;
use cosmwasm_std::Addr;
use prometheus_client::encoding::text::Encode;
use serde::de::DeserializeOwned;
use tendermint::block;
use terrars::mekapi::Mekapi;
use tokio::sync::Mutex;

use crate::metrics;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    V1PoolResponses,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    version: String,
    method: Method,
    result: metrics::ResultLabel,
}

pub struct Observed<L> {
    next: L,
    metrics: Arc<Mutex<metrics::Metrics>>,
    version: String,
}

#[async_trait::async_trait]
impl<L> Mekapi for Observed<L>
where
    L: Mekapi,
{
    async fn v1_pool_responses<T: Send + DeserializeOwned>(
        &self,
        factory: Addr,
        height: block::Height,
    ) -> Result<Vec<T>> {
        let start = Instant::now();
        let res = self.next.v1_pool_responses::<T>(factory, height).await;

        self.metrics
            .lock()
            .await
            .mekapi_call_duration
            .get_or_create(&Labels {
                version: self.version.clone(),
                method: Method::V1PoolResponses,
                result: if res.is_err() {
                    metrics::ResultLabel::Error
                } else {
                    metrics::ResultLabel::Success
                },
            })
            .observe(start.elapsed().as_secs_f64());

        res
    }
}
