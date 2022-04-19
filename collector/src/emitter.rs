use anyhow::Result;
use async_trait::async_trait;
use common::{
    metrics::{self, Registry, BUCKETS, BYTE_BUCKETS},
    storage,
    tracing::{debug, error},
};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Unit,
};
use tokio::{sync::mpsc, time::Instant};
use tracing::instrument;

use crate::segment::Segment;

#[async_trait]
pub trait Emitter {
    async fn emit(&self, segment: Segment) -> Result<storage::Metadata>;
}

#[instrument(skip(emitter, incoming), err)]
pub async fn run<E>(emitter: E, mut incoming: mpsc::Receiver<Segment>) -> Result<()>
where
    E: Emitter,
{
    while let Some(segment) = incoming.recv().await {
        match emitter.emit(segment).await {
            Ok(metadata) => debug!(msg = "succeeded to insert", metadata = ?metadata),
            Err(err) => error!(msg = "failed to insert", err = ?err),
        }
    }

    Ok(())
}

pub struct Gcs<B> {
    bucket: B,

    binary: String,
    hostname: String,
}

impl<B> Gcs<B>
where
    B: storage::Bucket + Send + Sync,
{
    pub fn new(binary: String, hostname: String, bucket: B) -> Self {
        Self {
            bucket,

            binary,
            hostname,
        }
    }
}

#[async_trait]
impl<B> Emitter for Gcs<B>
where
    B: storage::Bucket + Send + Sync,
{
    async fn emit(&self, segment: Segment) -> Result<storage::Metadata> {
        let body: Vec<u8> = segment
            .events
            .iter()
            .fold(Vec::new(), |mut body, (id, event)| {
                let mut buf = [0; ulid::ULID_LEN];
                id.to_str(&mut buf).unwrap();

                body.extend_from_slice(&buf);
                body.push(b' ');
                body.extend_from_slice(event.as_bytes());
                body.push(b'\n');
                body
            });

        let prefix = {
            let filename = segment.filename();
            let mut prefix = filename.split('.').collect::<Vec<_>>();
            prefix.pop();
            prefix.join("/")
        };

        self.bucket
            .insert(
                format!(
                    "{}/{}_{}_{}",
                    prefix,
                    segment.filename(),
                    self.binary,
                    self.hostname
                ),
                body.len().try_into().unwrap(),
                None,
                body.as_slice(),
            )
            .await
    }
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct Labels {
    bucket: String,
    result: metrics::ResultLabel,
}

pub struct Observed<N> {
    next: N,

    bucket: String,
    call_duration: Family<Labels, Histogram>,
    events: Family<Labels, Counter>,
    segment_size: Family<Labels, Histogram>,
}

impl<N> Observed<N> {
    pub fn new(registry: &mut Registry, bucket: String, next: N) -> Self {
        let call_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "emitter_call_duration",
            "Emitter method call durations",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );

        let events = Family::<Labels, Counter>::default();
        registry.register(
            "emitter_events",
            "Number of events emitted",
            Box::new(events.clone()),
        );

        let segment_size = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BYTE_BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "emitter_segment_size",
            "Size of segments in bytes",
            Unit::Bytes,
            Box::new(segment_size.clone()),
        );

        Self {
            next,

            bucket,
            call_duration,
            events,
            segment_size,
        }
    }
}

#[async_trait]
impl<N> Emitter for Observed<N>
where
    N: Emitter + Send + Sync,
{
    #[instrument(level = "debug", skip(self, segment), err, ret)]
    async fn emit(&self, segment: Segment) -> Result<storage::Metadata> {
        let events = segment.events.len();
        let start = Instant::now();
        let res = self.next.emit(segment).await;

        let labels = Labels {
            bucket: self.bucket.clone(),
            result: if res.is_ok() {
                metrics::ResultLabel::Success
            } else {
                metrics::ResultLabel::Error
            },
        };
        self.call_duration
            .get_or_create(&labels)
            .observe(start.elapsed().as_secs_f64());
        self.events
            .get_or_create(&labels)
            .inc_by(events.try_into().unwrap_or_default());

        if let Ok(metadata) = res.as_ref() {
            self.segment_size
                .get_or_create(&labels)
                .observe(metadata.size as f64);
        }

        res
    }
}
