use std::{future::Future, net::SocketAddr, sync::Arc};

use axum::{middleware, Router, Server};
use prometheus_client::{
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

use common::metrics;

pub fn bind(
    registry: &mut metrics::Registry,
    addr: impl Into<SocketAddr>,
    params_router: Router,
) -> impl Future<Output = Result<(), hyper::Error>> {
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

    let router = Router::new()
        .nest("/params", params_router)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn({
            let request_duration = Arc::clone(&request_duration);
            move |req, next| metrics::middleware(req, next, Arc::clone(&request_duration))
        }));

    Server::bind(&addr.into()).serve(router.into_make_service())
}
