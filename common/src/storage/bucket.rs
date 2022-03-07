use std::io;

use anyhow::Result;
use async_trait::async_trait;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tokio::time::Instant;

use crate::metrics::{Registry, BUCKETS};

#[async_trait]
pub trait Bucket {
    async fn insert<'body, B>(
        &self,
        name: String,
        content_length: u64,
        content_type: Option<String>,
        body: B,
    ) -> Result<Option<Id>>
    where
        B: io::Read + Send + 'body;
}

pub struct Id(String);

impl From<String> for Id {
    fn from(input: String) -> Self {
        Self(input)
    }
}

pub trait Labeled {
    const BACKEND: Backend;

    fn bucket_name(&self) -> String;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Backend {
    Gcs,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    Insert,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct Labels {
    backend: Backend,
    bucket_name: String,
    method: Method,
}

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
            "storage_bucket_call_duration",
            "Storage bucket call durations",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );

        Self {
            call_duration,
            next,
        }
    }
}

#[async_trait]
impl<N> Bucket for Observed<N>
where
    N: Bucket + Labeled + Send + Sync,
{
    async fn insert<'body, B>(
        &self,
        name: String,
        content_length: u64,
        content_type: Option<String>,
        body: B,
    ) -> Result<Option<Id>>
    where
        B: io::Read + Send + 'body,
    {
        let start = Instant::now();
        let res = self
            .next
            .insert(name, content_length, content_type, body)
            .await;

        self.call_duration
            .get_or_create(&Labels {
                backend: N::BACKEND,
                bucket_name: self.next.bucket_name(),
                method: Method::Insert,
            })
            .observe(start.elapsed().as_secs_f64());

        res
    }
}
