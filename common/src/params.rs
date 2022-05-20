use std::sync::Arc;

use axum::{middleware, Router};
use prometheus_client::{
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use crate::metrics;

pub fn routes(registry: &mut metrics::Registry, params_router: Router) -> Router {
    let request_duration =
        Family::<metrics::RequestLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(metrics::BUCKETS.clone().into_iter())
        });
    registry.register_with_unit(
        "params_http_duration",
        "params endpoint HTTP request latencies",
        Unit::Seconds,
        Box::new(request_duration.clone()),
    );
    let request_duration = Arc::new(Mutex::new(request_duration));

    Router::new()
        .nest("/", params_router)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn({
            let request_duration = Arc::clone(&request_duration);
            move |req, next| metrics::middleware(req, next, Arc::clone(&request_duration))
        }))
}
