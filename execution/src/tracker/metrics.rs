use anyhow::Result;
use common::metrics::{self, Registry};
use cosmwasm_std::Addr;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tendermint::{abci::transaction::Hash, Block};
use tokio::time::Instant;

use super::Tracker;
use crate::strategy::Route;

#[derive(Clone, Hash, Debug, Encode, Eq, PartialEq)]
enum Method {
    IsTracked,
    ObserveBlock,
    Track,
}

#[derive(Clone, Hash, Debug, Encode, Eq, PartialEq)]
struct CallLabels {
    method: Method,
}

pub struct Observed<T> {
    call_duration: Family<CallLabels, Histogram>,
    next: T,
}

impl<T> Observed<T> {
    pub fn new(registry: &mut Registry, next: T) -> Self {
        let call_duration = Family::<CallLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(metrics::BUCKETS.clone().into_iter())
        });

        registry.register_with_unit(
            "tracker_call_duration",
            "Total number of calls tracker methods",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );

        Self {
            call_duration,
            next,
        }
    }

    fn observe(&self, start: Instant, method: Method) {
        self.call_duration
            .get_or_create(&CallLabels { method })
            .observe(start.elapsed().as_secs_f64());
    }
}

impl<T> Tracker for Observed<T>
where
    T: Tracker,
{
    fn is_tracked(&self, route: &Route) -> Option<Addr> {
        let start = Instant::now();
        let res = self.next.is_tracked(route);
        self.observe(start, Method::IsTracked);
        res
    }

    fn observe_block(&mut self, block: Block) -> Result<()> {
        let start = Instant::now();
        let res = self.next.observe_block(block);
        self.observe(start, Method::ObserveBlock);
        res
    }

    fn track(&mut self, hash: Hash, route: &Route) -> Option<Route> {
        let start = Instant::now();
        let res = self.next.track(hash, route);
        self.observe(start, Method::Track);
        res
    }
}
