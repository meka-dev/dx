use std::time::Instant;

use async_trait::async_trait;
use common::{
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error},
    Report,
    Result,
};
use cosmwasm_std::Addr;
use futures::stream::BoxStream;
use mekapi::{pools, v1, v3};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tendermint::block;
use terrars::dex;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
enum Method {
    PoolsV4,
    V1PoolResponses,
    V1EventsMemprolTx,
    V1EventsMemprolTxV2,
    V3Pools,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct Labels {
    factory: String,
    method: Method,
    result: ResultLabel,
}

#[derive(Clone)]
pub struct Observed<N> {
    call_duration: Family<Labels, Histogram>,
    next: N,
}

impl<N> Observed<N> {
    pub fn new(registry: &mut Registry, next: N) -> Self {
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
        factory: Option<Addr>,
        height: block::Height,
        method: Method,
        err: Option<&Report>,
    ) {
        let elapsed = start.elapsed();
        let factory = factory.unwrap_or_else(|| Addr::unchecked(""));
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
impl<M> pools::Api for Observed<M>
where
    M: pools::Api,
{
    async fn v4(&self, height: block::Height) -> Result<Vec<pools::Pool>> {
        let start = Instant::now();
        let res = self.next.v4(height).await;

        self.observe(start, None, height, Method::PoolsV4, res.as_ref().err());

        res
    }
}

#[async_trait]
impl<N> v1::Api for Observed<N>
where
    N: v1::Api,
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
            Some(factory),
            height,
            Method::V1PoolResponses,
            res.as_ref().err(),
        );

        res
    }

    async fn v1_events_mempooltx(
        &self,
    ) -> Result<BoxStream<'static, Result<v1::MempoolTxMessage>>> {
        let start = Instant::now();
        let res = self.next.v1_events_mempooltx().await;

        self.observe(
            start,
            None,
            block::Height::default(),
            Method::V1EventsMemprolTx,
            res.as_ref().err(),
        );

        res
    }

    async fn v1_events_mempooltx_v2(
        &self,
    ) -> Result<BoxStream<'static, Result<v1::MempoolTxV2Message>>> {
        let start = Instant::now();
        let res = self.next.v1_events_mempooltx_v2().await;

        self.observe(
            start,
            None,
            block::Height::default(),
            Method::V1EventsMemprolTxV2,
            res.as_ref().err(),
        );

        res
    }
}

#[async_trait]
impl<N> v3::Api for Observed<N>
where
    N: v3::Api,
{
    async fn v3_pools(&self, factory: Addr, height: block::Height) -> Result<Vec<v3::Pool>> {
        let start = Instant::now();
        let res = self.next.v3_pools(factory.clone(), height).await;

        self.observe(
            start,
            Some(factory),
            height,
            Method::V3Pools,
            res.as_ref().err(),
        );

        res
    }
}
