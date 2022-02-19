use std::{sync::Arc, time::Instant};

use cosmwasm_std::Addr;
use prometheus_client::encoding::text::Encode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use terrars::{lcd::Lcd, AccountId, Coin};

use crate::metrics;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    BankBalances,
    WasmQuery,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum ResultLabel {
    Success,
    Error,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    version: String,
    method: Method,
    result: ResultLabel,
}

pub struct Observed<L> {
    next: L,
    metrics: Arc<Mutex<metrics::Metrics>>,
    version: String,
}

impl<L> Observed<L> {
    pub fn new(version: impl Into<String>, metrics: Arc<Mutex<metrics::Metrics>>, next: L) -> Self {
        Self {
            metrics,
            next,
            version: version.into(),
        }
    }

    async fn observe(&self, start: Instant, method: Method, result: ResultLabel) {
        self.metrics
            .lock()
            .await
            .lcd_call_duration
            .get_or_create(&Labels {
                version: self.version.clone(),
                method,
                result,
            })
            .observe(start.elapsed().as_secs_f64());
    }
}

#[async_trait::async_trait]
impl<L> Lcd for Observed<L>
where
    L: Lcd,
{
    async fn bank_balances(&self, account_id: &AccountId) -> anyhow::Result<Vec<Coin>> {
        let start = Instant::now();
        let res = self.next.bank_balances(account_id).await;
        self.observe(
            start,
            Method::BankBalances,
            if res.is_err() {
                ResultLabel::Error
            } else {
                ResultLabel::Success
            },
        )
        .await;

        res
    }

    async fn wasm_query<Q: ?Sized + Sync + Serialize, T: Send + for<'de> Deserialize<'de>>(
        &self,
        contract_addr: &Addr,
        query: &Q,
    ) -> anyhow::Result<T> {
        let start = Instant::now();
        let res = self.next.wasm_query(contract_addr, query).await;
        self.observe(
            start,
            Method::WasmQuery,
            if res.is_err() {
                ResultLabel::Error
            } else {
                ResultLabel::Success
            },
        )
        .await;

        res
    }
}
