use std::time::Instant;

use anyhow::{Error, Result};
use async_trait::async_trait;
use cosmwasm_std::Addr;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tendermint::block;

use common::{
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error},
};
use terrars::{
    dex,
    mekapi::{v3, Mekapi},
};

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
enum Method {
    V1PoolResponses,
    V3Pools,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct Labels {
    factory: String,
    method: Method,
    result: ResultLabel,
}

#[derive(Clone)]
pub struct Observed<L> {
    call_duration: Family<Labels, Histogram>,
    next: L,
}

impl<L> Observed<L> {
    pub fn new(registry: &mut Registry, next: L) -> Self {
        let call_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "mekapi_call_duration",
            "Mekapi call latency",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );

        Self {
            call_duration,
            next,
        }
    }

    fn observe(
        &self,
        start: Instant,
        factory: Addr,
        height: block::Height,
        method: Method,
        err: Option<&Error>,
    ) {
        let elapsed = start.elapsed();
        self.call_duration
            .get_or_create(&Labels {
                factory: factory.to_string(),
                method: method.clone(),
                result: if err.is_some() {
                    ResultLabel::Error
                } else {
                    ResultLabel::Success
                },
            })
            .observe(elapsed.as_secs_f64());

        match err {
            None => debug!(
                "method={:?} factory={} height={} elapsed={:?}",
                method,
                factory.to_string(),
                height.to_string(),
                elapsed,
            ),
            Some(err) => error!(
                "method={:?} factory={} height={} elapsed={:?} error={}",
                method,
                factory.to_string(),
                height.to_string(),
                elapsed,
                err,
            ),
        }
    }
}

#[async_trait]
impl<M> Mekapi for Observed<M>
where
    M: Mekapi,
{
    async fn v1_pool_responses(
        &self,
        factory: Addr,
        height: block::Height,
    ) -> Result<Vec<dex::Pool>> {
        let start = Instant::now();
        let res = self.next.v1_pool_responses(factory.clone(), height).await;

        self.observe(
            start,
            factory,
            height,
            Method::V1PoolResponses,
            res.as_ref().err(),
        );

        res
    }
}

#[async_trait]
impl<M> v3::Api for Observed<M>
where
    M: v3::Api,
{
    async fn v3_pools(&self, factory: Addr, height: block::Height) -> Result<Vec<v3::Pool>> {
        let start = Instant::now();
        let res = self.next.v3_pools(factory.clone(), height).await;

        self.observe(start, factory, height, Method::V3Pools, res.as_ref().err());

        res
    }
}
