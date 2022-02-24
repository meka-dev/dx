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

use crate::{broadcast, strategy};

pub mod lcd;
pub mod mekapi;

const HEADER_OPENMETRICS: &str = "application/openmetrics-text; version=1.0.0; charset=utf-8";
const ORG_NAMESPACE: &str = "mekatek";
const BINARY_NAMESPACE: &str = "execution";

lazy_static! {
    static ref BUCKETS: Vec<f64> = vec![
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

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct HttpLabels {
    method: String,
    path: String,
    status: String,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub struct LatestHeightLabels {
    pub version: String,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum ResultLabel {
    Success,
    Error,
}

pub struct Metrics {
    http_request_duration: Family<HttpLabels, Histogram>,
    pub latest_block_height: Family<LatestHeightLabels, Gauge>,
    pub lcd_call_duration: Family<lcd::Labels, Histogram>,
    pub mekapi_call_duration: Family<mekapi::Labels, Histogram>,
    pub strategy_duration: Family<strategy::metrics::DurationLabels, Histogram>,
    pub strategy_size_max: Family<strategy::metrics::TradesLabel, Gauge>,
    pub strategy_expected_profit_max: Family<strategy::metrics::TradesLabel, Gauge>,
    pub strategy_trades: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_size_uusd: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_expected_profit_uusd: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_messages_emitted: Family<strategy::metrics::TradesLabel, Counter>,
    pub strategy_trade_insufficient_balance: Family<strategy::metrics::TradesLabel, Counter>,
    pub broadcast_estimated_fee_uusd: Family<broadcast::Labels, Counter>,
    pub broadcast_expected_profit_after_fee_uusd: Family<broadcast::Labels, Counter>,
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
        let latest_block_height = Family::<LatestHeightLabels, Gauge>::default();
        let lcd_call_duration = Family::<lcd::Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        let mekapi_call_duration =
            Family::<mekapi::Labels, Histogram>::new_with_constructor(|| {
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
        let broadcast_estimated_fee_uusd = Family::<broadcast::Labels, Counter>::default();
        let broadcast_expected_profit_after_fee_uusd =
            Family::<broadcast::Labels, Counter>::default();

        registry.register(
            prefix("http_request_duration_seconds"),
            "HTTP request durations",
            Box::new(http_request_duration.clone()),
        );
        registry.register(
            prefix("latest_block_height"),
            "Latest observed block height",
            Box::new(latest_block_height.clone()),
        );
        registry.register(
            prefix("lcd_call_duration_seconds"),
            "LCD call latency",
            Box::new(lcd_call_duration.clone()),
        );
        registry.register(
            prefix("mekapi_call_duration_seconds"),
            "Mekapi call latency",
            Box::new(mekapi_call_duration.clone()),
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
        registry.register(
            prefix("broadcast_estimated_fee_uusd"),
            "Estimated transaction fee",
            Box::new(broadcast_estimated_fee_uusd.clone()),
        );
        registry.register(
            prefix("broadcast_expected_profit_after_fee_uusd"),
            "Expected profit after fees have been subtracted",
            Box::new(broadcast_expected_profit_after_fee_uusd.clone()),
        );

        let registry = Arc::new(Mutex::new(registry));
        let metrics = Self {
            http_request_duration,
            latest_block_height,
            lcd_call_duration,
            mekapi_call_duration,
            strategy_duration,
            strategy_size_max,
            strategy_expected_profit_max,
            strategy_trades,
            strategy_trade_size_uusd,
            strategy_trade_expected_profit_uusd,
            strategy_trade_messages_emitted,
            strategy_trade_insufficient_balance,
            broadcast_estimated_fee_uusd,
            broadcast_expected_profit_after_fee_uusd,
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
