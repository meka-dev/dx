use std::{fmt, sync::Arc};

use axum::{
    extract::{Extension, MatchedPath},
    http::Request,
    middleware,
    middleware::Next,
    response::IntoResponse,
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use prometheus_client::{
    encoding::text::{encode, Encode, SendSyncEncodeMetric},
    metrics::{family::Family, histogram::Histogram},
    registry::{self, Unit},
};
use tokio::{sync::Mutex, time::Instant};

const HEADER_OPENMETRICS: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";
const ORG_NAMESPACE: &str = "mekatek";

lazy_static! {
    pub static ref BUCKETS: Vec<f64> = vec![
        0.000100, // 100us
        0.000250, // 250us
        0.000500, // 500us
        0.001000, // 1ms
        0.002500, // 2.5ms
        0.005000, // 5ms
        0.010000, // 10ms
        0.025000, // 25ms
        0.050000, // 50ms
        0.100000, // 100ms
        0.200000, // 200ms
        0.300000, // 300ms
        0.400000, // 400ms
        0.500000, // 500ms
        0.600000, // 600ms
        0.700000, // 700ms
        0.800000, // 800ms
        0.900000, // 900ms
        1.000000, // 1s
        2.000000, // 2s
        3.000000, // 3s
    ];
}

lazy_static! {
    pub static ref BYTE_BUCKETS: Vec<f64> = vec![
                  100.0, // 100b
           1.0 * 1024.0, // 1kb
          10.0 * 1024.0, // 10kb
         100.0 * 1024.0, // 100kb
         250.0 * 1024.0, // 250kb
         500.0 * 1024.0, // 500kb
         750.0 * 1024.0, // 750kb
        1000.0 * 1024.0, // 1mb
        1250.0 * 1024.0, // 1.25mb
        1500.0 * 1024.0, // 1.50mb
        1750.0 * 1024.0, // 1.75mb
        2000.0 * 1024.0, // 2mb
    ];
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct RequestLabels {
    method: String,
    path: String,
    status: String,
}

pub type RequestDuration = Family<RequestLabels, Histogram>;

pub fn new_request_duration() -> RequestDuration {
    Family::<RequestLabels, Histogram>::new_with_constructor(|| {
        Histogram::new(BUCKETS.clone().into_iter())
    })
}

pub struct Registry {
    binary: &'static str,
    registry: registry::Registry,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum ResultLabel {
    Success,
    Error,
}

impl Registry {
    pub fn new(binary: &'static str) -> Self {
        Self {
            binary,
            registry: <registry::Registry>::default(),
        }
    }

    pub fn register<N, H>(
        &mut self,
        name: N,
        help: H,
        metric: Box<dyn SendSyncEncodeMetric + 'static>,
    ) where
        N: Into<String>,
        H: Into<String>,
    {
        self.registry
            .register(prefix(self.binary, name), help, metric)
    }

    pub fn register_with_unit<N, H>(
        &mut self,
        name: N,
        help: H,
        unit: Unit,
        metric: Box<dyn SendSyncEncodeMetric + 'static>,
    ) where
        N: Into<String>,
        H: Into<String>,
    {
        self.registry
            .register_with_unit(prefix(self.binary, name), help, unit, metric)
    }
}

pub fn routes<S: Into<String> + fmt::Display>(mut registry: Registry, binary: S) -> Router {
    let request_duration = new_request_duration();
    registry.registry.register_with_unit(
        prefix(binary, "metrics_http_duration"),
        "metrics endpoint HTTP request latencies",
        Unit::Seconds,
        Box::new(request_duration.clone()),
    );

    let request_duration = Arc::new(Mutex::new(request_duration));
    let registry = Arc::new(Mutex::new(registry));

    Router::new()
        .route("/", get(handler))
        .layer(Extension(registry))
        .layer(middleware::from_fn({
            let request_duration = Arc::clone(&request_duration);
            move |req, next| middleware(req, next, Arc::clone(&request_duration))
        }))
}

pub async fn middleware<B>(
    req: Request<B>,
    next: Next<B>,
    request_duration: Arc<Mutex<RequestDuration>>,
) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().to_string();

    let response = next.run(req).await;

    request_duration
        .lock()
        .await
        .get_or_create(&RequestLabels {
            path,
            method,
            status: response.status().as_str().to_string(),
        })
        .observe(start.elapsed().as_secs_f64());

    response
}

async fn handler(Extension(registry): Extension<Arc<Mutex<Registry>>>) -> impl IntoResponse {
    let mut encoded = Vec::new();
    encode(&mut encoded, &registry.lock().await.registry).unwrap();

    ([("Content-Type", HEADER_OPENMETRICS)], encoded)
}

fn prefix<B: Into<String> + fmt::Display, S: Into<String>>(binary: B, suffix: S) -> String {
    format!("{}_{}_{}", ORG_NAMESPACE, binary, suffix.into())
}
