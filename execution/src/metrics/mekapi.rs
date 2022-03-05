use std::time::Instant;

use anyhow::Result;
use cosmwasm_std::Addr;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tendermint::block;
use terrars::mekapi::Mekapi;

use common::metrics::{Registry, ResultLabel, BUCKETS};
use terrars::dex;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
enum Method {
    V1PoolResponses,
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

        self.call_duration
            .get_or_create(&Labels {
                factory: factory.to_string(),
                method: Method::V1PoolResponses,
                result: if res.is_err() {
                    ResultLabel::Error
                } else {
                    ResultLabel::Success
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
