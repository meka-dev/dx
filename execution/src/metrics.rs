use std::{future::Future, net::SocketAddr, sync::Arc};

use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, gauge::Gauge, histogram::Histogram},
    registry::Unit,
};
use tokio::sync::Mutex;

use common::metrics::BUCKETS;

use crate::{broadcast, strategy};

pub mod lcd;
pub mod mekapi;

const BINARY_NAMESPACE: &str = "execution";

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
    pub latest_block_height: Family<LatestHeightLabels, Gauge>,
    pub lcd_call_duration: Family<lcd::Labels, Histogram>,
    pub mekapi_call_duration: Family<mekapi::Labels, Histogram>,
    pub fetcher_pools_duration: Family<strategy::fetcher::Labels, Histogram>,
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
        let latest_block_height = Family::<LatestHeightLabels, Gauge>::default();
        let lcd_call_duration = Family::<lcd::Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        let mekapi_call_duration =
            Family::<mekapi::Labels, Histogram>::new_with_constructor(|| {
                Histogram::new(BUCKETS.clone().into_iter())
            });
        let fetcher_pools_duration =
            Family::<strategy::fetcher::Labels, Histogram>::new_with_constructor(|| {
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

        let mut registry = common::metrics::Registry::new(BINARY_NAMESPACE.to_string());
        registry.register(
            "latest_block_height",
            "Latest observed block height",
            Box::new(latest_block_height.clone()),
        );
        registry.register_with_unit(
            "lcd_call_duration_seconds",
            "LCD call latency",
            Unit::Seconds,
            Box::new(lcd_call_duration.clone()),
        );
        registry.register_with_unit(
            "mekapi_call_duration",
            "Mekapi call latency",
            Unit::Seconds,
            Box::new(mekapi_call_duration.clone()),
        );
        registry.register_with_unit(
            "fetcher_pools_duration",
            "Fetcher pools retrieval durations",
            Unit::Seconds,
            Box::new(strategy_duration.clone()),
        );
        registry.register_with_unit(
            "strategy_duration",
            "Strategy durations",
            Unit::Seconds,
            Box::new(strategy_duration.clone()),
        );
        registry.register(
            "strategy_size_max",
            "Largest trade size within a strategy execution",
            Box::new(strategy_size_max.clone()),
        );
        registry.register(
            "strategy_expected_profit_max",
            "Largest trade profit within a strategy execution",
            Box::new(strategy_expected_profit_max.clone()),
        );
        registry.register(
            "strategy_trades",
            "Number of transactions emitted by strategies",
            Box::new(strategy_trades.clone()),
        );
        registry.register(
            "strategy_trade_size_uusd",
            "Liquidity required to execute optimal strategy trades",
            Box::new(strategy_trade_size_uusd.clone()),
        );
        registry.register(
            "strategy_trade_expected_profit_uusd",
            "Profit expected from trades made by strategies",
            Box::new(strategy_trade_expected_profit_uusd.clone()),
        );
        registry.register(
            "strategy_trade_messages_emitted",
            "Number of messages emitted by strategies",
            Box::new(strategy_trade_messages_emitted.clone()),
        );
        registry.register(
            "strategy_trade_insufficient_balance",
            "Number of trades which didn't produce messages because of lack of funds",
            Box::new(strategy_trade_insufficient_balance.clone()),
        );
        registry.register(
            "broadcast_estimated_fee_uusd",
            "Estimated transaction fee",
            Box::new(broadcast_estimated_fee_uusd.clone()),
        );
        registry.register(
            "broadcast_expected_profit_after_fee_uusd",
            "Expected profit after fees have been subtracted",
            Box::new(broadcast_expected_profit_after_fee_uusd.clone()),
        );

        let metrics = Self {
            latest_block_height,
            lcd_call_duration,
            mekapi_call_duration,
            fetcher_pools_duration,
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

        let (_, server) = common::metrics::bind(addr, BINARY_NAMESPACE, registry);

        (metrics, server)
    }
}
