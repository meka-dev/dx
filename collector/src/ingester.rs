use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    time::Duration,
};

use anyhow::{anyhow, Result};
use common::{
    metrics::{Registry, BUCKETS},
    tracing::{error, trace, warn},
};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Unit,
};
use tokio::{
    select,
    sync::mpsc::{self, error::TrySendError},
    time::{self, Instant},
};
use tracing::instrument;

use crate::segment::{Key, Segment};

pub trait Ingester {
    fn collect(&mut self, segemnt: Segment) -> (usize, usize);
    fn flush(&mut self) -> Vec<Segment>;
}

#[derive(Debug)]
pub struct RunConfig {
    pub flush_interval: Duration,
}

#[instrument(skip(ingester, incoming, outgoing), err)]
pub async fn run<I>(
    flush_interval: Duration,
    mut ingester: I,
    mut incoming: mpsc::Receiver<Segment>,
    outgoing: mpsc::Sender<Segment>,
) -> Result<()>
where
    I: Ingester,
{
    let mut flush = time::interval(flush_interval);

    loop {
        select! {
            biased;

            _ = flush.tick() => {
                let segments = ingester.flush();

                trace!(branch = "flush", segments = segments.len());
                for segment in segments {
                    match outgoing.try_send(segment) {
                        Ok(_) => {},
                        Err(TrySendError::Full(_)) => {
                            warn!(branch = "flush", msg = "failed to send segment, dropping");
                        },
                        Err(TrySendError::Closed(_)) => {
                            error!(branch = "flush", msg = "emitter is gone");
                            return Err(anyhow!("emitter is gone"));
                        }
                    }
                }
            },

            Some(segment) = incoming.recv() => {
                trace!(branch = "incoming", namespace = ?segment.namespace);
                ingester.collect(segment);
            },

            else => break,
        }
    }

    Ok(())
}

pub struct InMemory {
    segments: BTreeMap<Key, Segment>,

    size_flushes: BTreeSet<Key>,
    time_flushes: Vec<(Instant, Key)>,

    count_limit: usize,
    size_limit: usize,
    time_limit: Duration,
}

impl InMemory {
    pub fn new(size_limit: usize, count_limit: usize, time_limit: Duration) -> Self {
        Self {
            segments: Default::default(),

            size_flushes: Default::default(),
            time_flushes: Default::default(),

            count_limit,
            size_limit,
            time_limit,
        }
    }
}

impl Ingester for InMemory {
    fn collect(&mut self, segment: Segment) -> (usize, usize) {
        let key = Key::from(&segment);
        let (count, size) = match self.segments.entry(key.clone()) {
            Entry::Vacant(entry) => {
                let count = segment.events.len();
                let size = segment.size;
                entry.insert(segment);
                self.time_flushes.push((Instant::now(), key.clone()));

                (count, size)
            },
            Entry::Occupied(mut entry) => match entry.get_mut().merge(segment) {
                Ok((count, size)) => (count, size),
                Err(_err) => unreachable!(),
            },
        };

        if count >= self.count_limit {
            let _ = self.size_flushes.insert(key.clone());
        }
        if size >= self.size_limit {
            let _ = self.size_flushes.insert(key);
        }

        (count, size)
    }

    fn flush(&mut self) -> Vec<Segment> {
        let mut segments = Vec::new();

        for key in &self.size_flushes {
            if let Some(segment) = self.segments.remove(key) {
                segments.push(segment);
            }
        }
        self.size_flushes = BTreeSet::new();

        self.time_flushes.retain(|(created, key)| {
            if Instant::now() - *created > self.time_limit {
                if let Some(segment) = self.segments.remove(key) {
                    segments.push(segment);
                }
                return false;
            }

            true
        });

        segments
    }
}

impl Labeled for InMemory {
    const NAME: Name = Name::InMemory;
}

pub trait Labeled {
    const NAME: Name;
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
enum Direction {
    Ingress,
    Egress,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
enum Method {
    Collect,
    Flush,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    InMemory,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    method: Method,
    name: Name,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct CounterLabels {
    direction: Direction,
    name: Name,
}

pub struct Observed<N> {
    next: N,

    call_duration: Family<Labels, Histogram>,
    events: Family<CounterLabels, Counter>,
}

impl<N> Observed<N>
where
    N: Labeled,
{
    pub fn new(registry: &mut Registry, next: N) -> Self {
        let call_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "ingester_call_duration",
            "Ingester method call durations",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );
        let events = Family::<CounterLabels, Counter>::default();
        registry.register(
            "ingester_events",
            "Ingress and Egress counters of events",
            Box::new(events.clone()),
        );

        Self {
            next,

            call_duration,
            events,
        }
    }

    fn observe(&self, start: Instant, method: Method) {
        self.call_duration
            .get_or_create(&Labels {
                method,
                name: N::NAME,
            })
            .observe(start.elapsed().as_secs_f64());
    }
}

impl<N> Ingester for Observed<N>
where
    N: Ingester + Labeled,
{
    #[instrument(level = "trace", skip(self), ret)]
    fn collect(&mut self, segment: Segment) -> (usize, usize) {
        let events = segment.events.len();
        let start = Instant::now();
        let res = self.next.collect(segment);

        self.observe(start, Method::Collect);

        let counter_labels = CounterLabels {
            direction: Direction::Ingress,
            name: N::NAME,
        };
        self.events
            .get_or_create(&counter_labels)
            .inc_by(events.try_into().unwrap_or_default());

        res
    }

    #[instrument(level = "trace", skip(self), ret)]
    fn flush(&mut self) -> Vec<Segment> {
        let start = Instant::now();
        let segments = self.next.flush();

        self.observe(start, Method::Flush);

        let events = segments.iter().fold(0, |len, seg| len + seg.events.len());
        self.events
            .get_or_create(&CounterLabels {
                direction: Direction::Egress,
                name: N::NAME,
            })
            .inc_by(events.try_into().unwrap_or_default());

        segments
    }
}
