use std::sync::Arc;

use cosmwasm_bignumber::Decimal256;
use prometheus_client::encoding::text::Encode;
use tokio::sync::Mutex;

use crate::{config, metrics::Metrics};

#[derive(Clone, Copy, Debug, Hash, Encode, Eq, PartialEq)]
enum Outcome {
    Profitable,
    NotProfitable,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    version: String,
    outcome: Outcome,
}

pub async fn observe(
    metrics: Arc<Mutex<Metrics>>,
    messages_len: usize,
    expected_profit: Decimal256,
    fee: Decimal256,
) {
    let outcome = if fee <= expected_profit {
        Outcome::NotProfitable
    } else {
        Outcome::Profitable
    };
    let metrics = metrics.lock().await;
    metrics
        .broadcast_estimated_fee_uusd
        .get_or_create(&Labels {
            version: config::VERSION.to_string(),
            outcome,
        })
        .inc_by((fee.0 / 10u64.pow(12).into()).as_u64());

    log::info!(
        "messages={} expected_profit={} estimated_fee={} outcome={:?}",
        messages_len,
        expected_profit,
        fee,
        outcome,
    );
}
