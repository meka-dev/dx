use std::sync::Arc;

use anyhow::Result;
use prometheus_client::encoding::text::Encode;
use tokio::sync::Mutex;

use terrars::{base::abci::GasInfo, lcd, tx};

use crate::metrics::Metrics;

#[derive(Clone, Copy, Debug, Hash, Encode, Eq, PartialEq)]
enum Outcome {
    Profitable,
    NotProfitable,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    outcome: Outcome,
}

pub async fn observe(
    metrics: Arc<Mutex<Metrics>>,
    messages_len: usize,
    expected_profit: u64,
    fee: u64,
) {
    let outcome = if fee <= expected_profit {
        Outcome::Profitable
    } else {
        Outcome::NotProfitable
    };
    let metrics = metrics.lock().await;
    metrics
        .broadcast_estimated_fee_uusd
        .get_or_create(&Labels { outcome })
        .inc_by(fee);

    log::info!(
        "messages={} expected_profit={} estimated_fee={} outcome={:?}",
        messages_len,
        expected_profit,
        fee,
        outcome,
    );
}

// TODO(xla): Fetch gas prices.
pub async fn simulate<L>(
    lcd: &L,
    metrics: Arc<Mutex<Metrics>>,
    tx: &[u8],
    messages_len: usize,
    expected_profit: u64,
) -> Result<GasInfo>
where
    L: lcd::Lcd,
{
    match lcd.tx_simulate(tx).await? {
        lcd::Response::Ok(tx::SimulateResponse { gas_info, .. }) => {
            let fee = (gas_info.gas_used as f64 * 0.18) as u64;
            observe(Arc::clone(&metrics), messages_len, expected_profit, fee).await;

            Ok(gas_info)
        },
        lcd::Response::Error(lcd::Error { message, .. }) => {
            // TODO(xla): Emit as metric.
            log::error!("tx simulation failed: {}", message);
            Err(anyhow::anyhow!("tx simulation faield: {}", message))
        },
    }
}
