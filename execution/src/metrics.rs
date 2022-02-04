use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Extension, MatchedPath},
    http::Request,
    response::{Headers, IntoResponse},
    routing::get,
    AddExtensionLayer, Router, Server,
};
use axum_extra::{middleware, middleware::Next};
use prometheus_client::{
    encoding::text::{encode, Encode},
    metrics::{family::Family, histogram::Histogram},
    registry::Registry,
};
use tokio::{sync::Mutex, time::Instant};

const HEADER_OPENMETRICS: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct Labels {
    method: String,
    path: String,
    status: String,
}

pub async fn server(addr: impl Into<SocketAddr>) -> Result<(), hyper::Error> {
    let mut registry = <Registry>::default();

    let http_request_duration_seconds = Family::<Labels, Histogram>::new_with_constructor(|| {
        let buckets: Vec<f64> = vec![
            0.0001, 0.00025, 0.0005, 0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0,
            2.0, 3.0,
        ];
        Histogram::new(buckets.into_iter())
    });

    registry.register(
        "http_request_duration_seconds",
        "HTTP request durations",
        Box::new(http_request_duration_seconds.clone()),
    );

    let registry = Arc::new(Mutex::new(registry));
    let http_request_duration_seconds = Arc::new(http_request_duration_seconds);

    let metrics = Router::new()
        .route("/metrics", get(handler))
        .layer(AddExtensionLayer::new(registry))
        .layer(middleware::from_fn({
            move |req, next| track(req, next, Arc::clone(&http_request_duration_seconds))
        }));

    Server::bind(&addr.into())
        .serve(metrics.into_make_service())
        .await
}

async fn handler(Extension(registry): Extension<Arc<Mutex<Registry>>>) -> impl IntoResponse {
    let mut encoded = Vec::new();
    encode(&mut encoded, &(*registry.lock().await)).unwrap();

    (Headers(vec![("Content-Type", HEADER_OPENMETRICS)]), encoded)
}

async fn track<B>(
    req: Request<B>,
    next: Next<B>,
    http_request_duration_seconds: Arc<Family<Labels, Histogram>>,
) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().to_string();

    let response = next.run(req).await;

    http_request_duration_seconds
        .get_or_create(&Labels {
            path,
            method,
            status: response.status().as_str().to_string(),
        })
        .observe(start.elapsed().as_secs_f64());

    response
}
