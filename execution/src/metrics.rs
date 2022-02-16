use std::{fmt, future::Future, net::SocketAddr, sync::Arc};

use axum::{
    extract::{Extension, MatchedPath},
    http::Request,
    response::{Headers, IntoResponse},
    routing::get,
    AddExtensionLayer, Router, Server,
};
use axum_extra::{middleware, middleware::Next};
use lazy_static::lazy_static;
use prometheus_client::{
    encoding::text::{encode, Encode},
    metrics::{counter::Counter, family::Family, gauge::Gauge, histogram::Histogram},
    registry::Registry,
};
use tokio::{sync::Mutex, time::Instant};

use crate::{config, strategy};

const HEADER_OPENMETRICS: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";
const ORG_NAMESPACE: &str = "mekatek";
const BINARY_NAMESPACE: &str = "execution";

lazy_static! {
    static ref BUCKETS: Vec<f64> = vec![
        0.0001, 0.00025, 0.0005, 0.001, 0.0025, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.0,
        3.0,
    ];
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct HttpLabels {
    version: String,
    method: String,
    path: String,
    status: String,
}

pub struct Metrics {
    http_request_duration: Family<HttpLabels, Histogram>,
    pub strategy_duration: Family<strategy::metrics::DurationLabels, Histogram>,
    pub strategy_size_max: Family<strategy::metrics::TradesLabel, Gauge>,
    pub strategy_expected_profit_max: Family<strategy::metrics::TradesLabel, Gauge>,
    pub strategy_trades: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_size_uusd: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_expected_profit_uusd: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_messages_emitted: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_insufficient_balance: Family<strategy::metrics::TradesLabel, Counter>,
}

impl Metrics {
    pub fn new(
        addr: impl Into<SocketAddr>,
    ) -> (
        Arc<Mutex<Self>>,
        impl Future<Output = Result<(), hyper::Error>>,
    ) {
        let mut registry = <Registry>::default();
        let http_request_duration = Family::<HttpLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        let strategy_duration =
            Family::<strategy::metrics::DurationLabels, Histogram>::new_with_constructor(|| {
                Histogram::new(BUCKETS.clone().into_iter())
            });
        let strategy_size_max = Family::<strategy::metrics::TradesLabel, Gauge>::default();
        let strategy_expected_profit_max =
            Family::<strategy::metrics::TradesLabel, Gauge>::default();
        let strategy_trades = Family::<strategy::metrics::TradesLabel, Counter>::default();
        let strategy_trade_size_uusd = Family::<strategy::metrics::TradesLabel, Counter>::default();
        let strategy_trade_expected_profit_uusd =
            Family::<strategy::metrics::TradesLabel, Counter>::default();
        let strategy_trade_messages_emitted =
            Family::<strategy::metrics::TradesLabel, Counter>::default();
        let strategy_trade_insufficient_balance =
            Family::<strategy::metrics::TradesLabel, Counter>::default();

        registry.register(
            prefix("http_request_duration_seconds"),
            "HTTP request durations",
            Box::new(http_request_duration.clone()),
        );
        registry.register(
            prefix("strategy_duration_seconds"),
            "Strategy durations",
            Box::new(strategy_duration.clone()),
        );
        registry.register(
            prefix("strategy_size_max"),
            "Largest trade size within a strategy execution",
            Box::new(strategy_size_max.clone()),
        );
        registry.register(
            prefix("strategy_expected_profit_max"),
            "Largest trade profit within a strategy execution",
            Box::new(strategy_expected_profit_max.clone()),
        );
        registry.register(
            prefix("strategy_trades"),
            "Number of transactions emitted by strategies",
            Box::new(strategy_trades.clone()),
        );
        registry.register(
            prefix("strategy_trade_size_uusd"),
            "Liquidity required to execute optimal strategy trades",
            Box::new(strategy_trade_size_uusd.clone()),
        );
        registry.register(
            prefix("strategy_trade_expected_profit_uusd"),
            "Profit expected from trades made by strategies",
            Box::new(strategy_trade_expected_profit_uusd.clone()),
        );
        registry.register(
            prefix("strategy_trade_messages_emitted"),
            "Number of messages emitted by strategies",
            Box::new(strategy_trade_messages_emitted.clone()),
        );
        registry.register(
            prefix("strategy_trade_insufficient_balance"),
            "Number of trades which didn't produce messages because of lack of funds",
            Box::new(strategy_trade_insufficient_balance.clone()),
        );

        let registry = Arc::new(Mutex::new(registry));
        let metrics = Self {
            http_request_duration,
            strategy_duration,
            strategy_size_max,
            strategy_expected_profit_max,
            strategy_trades,
            strategy_trade_size_uusd,
            strategy_trade_expected_profit_uusd,
            strategy_trade_messages_emitted,
            strategy_trade_insufficient_balance,
        };
        let metrics = Arc::new(Mutex::new(metrics));

        let router = Router::new()
            .route("/metrics", get(handler))
            .layer(AddExtensionLayer::new(registry))
            .layer(middleware::from_fn({
                let metrics = Arc::clone(&metrics);
                move |req, next| track(req, next, Arc::clone(&metrics))
            }));

        (
            metrics,
            Server::bind(&addr.into()).serve(router.into_make_service()),
        )
    }
}

async fn handler(Extension(registry): Extension<Arc<Mutex<Registry>>>) -> impl IntoResponse {
    let mut encoded = Vec::new();
    encode(&mut encoded, &(*registry.lock().await)).unwrap();

    (Headers(vec![("Content-Type", HEADER_OPENMETRICS)]), encoded)
}

async fn track<B>(
    req: Request<B>,
    next: Next<B>,
    metrics: Arc<Mutex<Metrics>>,
) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().to_string();

    let response = next.run(req).await;

    metrics
        .lock()
        .await
        .http_request_duration
        .get_or_create(&HttpLabels {
            version: config::VERSION.to_string(),
            path,
            method,
            status: response.status().as_str().to_string(),
        })
        .observe(start.elapsed().as_secs_f64());

    response
}

fn prefix<S: Into<String> + fmt::Display>(suffix: S) -> String {
    format!("{}_{}_{}", ORG_NAMESPACE, BINARY_NAMESPACE, suffix)
}
