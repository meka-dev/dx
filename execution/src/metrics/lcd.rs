use std::{sync::Arc, time::Instant};

use anyhow::Result;
use cosmwasm_std::Addr;
use prometheus_client::encoding::text::Encode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use terrars::{
    lcd::{self, Lcd, Response},
    tx, AccountId, BaseAccount, Coin,
};

use crate::metrics;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    AuthAccount,
    BankBalances,
    TxComputeTax,
    TxSimulate,
    WasmQuery,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    method: Method,
    result: metrics::ResultLabel,
}

#[derive(Clone)]
pub struct Observed<L>
where
    L: Clone,
{
    next: L,
    metrics: Arc<Mutex<metrics::Metrics>>,
}

impl<L> Observed<L>
where
    L: Clone,
{
    pub fn new(metrics: Arc<Mutex<metrics::Metrics>>, next: L) -> Self {
        Self { metrics, next }
    }

    async fn observe(&self, start: Instant, method: Method, result: metrics::ResultLabel) {
        self.metrics
            .lock()
            .await
            .lcd_call_duration
            .get_or_create(&Labels { method, result })
            .observe(start.elapsed().as_secs_f64());
    }
}

#[async_trait::async_trait]
impl<L> Lcd for Observed<L>
where
    L: Clone + Lcd,
{
    async fn auth_account(&self, account_id: &AccountId) -> Result<Option<BaseAccount>> {
        let start = Instant::now();
        let res = self.next.auth_account(account_id).await;
        self.observe(
            start,
            Method::AuthAccount,
            if res.is_err() {
                metrics::ResultLabel::Error
            } else {
                metrics::ResultLabel::Success
            },
        )
        .await;

        res
    }

    async fn bank_balances(&self, account_id: &AccountId) -> Result<Vec<Coin>> {
        let start = Instant::now();
        let res = self.next.bank_balances(account_id).await;
        self.observe(
            start,
            Method::BankBalances,
            if res.is_err() {
                metrics::ResultLabel::Error
            } else {
                metrics::ResultLabel::Success
            },
        )
        .await;

        res
    }

    async fn tx_compute_tax(&self, tx: &[u8]) -> Result<lcd::Response<Vec<Coin>>> {
        let start = Instant::now();
        let res = self.next.tx_compute_tax(tx).await;
        self.observe(
            start,
            Method::TxComputeTax,
            if res.is_err() {
                metrics::ResultLabel::Error
            } else {
                metrics::ResultLabel::Success
            },
        )
        .await;

        res
    }

    async fn tx_simulate(&self, tx: &[u8]) -> Result<Response<tx::SimulateResponse>> {
        let start = Instant::now();
        let res = self.next.tx_simulate(tx).await;
        self.observe(
            start,
            Method::TxSimulate,
            if res.is_err() {
                metrics::ResultLabel::Error
            } else {
                metrics::ResultLabel::Success
            },
        )
        .await;

        res
    }

    async fn wasm_contract_store<
        Q: ?Sized + Sync + Serialize,
        T: Send + for<'de> Deserialize<'de>,
    >(
        &self,
        contract_addr: &Addr,
        query_msg: &Q,
    ) -> Result<T> {
        let start = Instant::now();
        let res = self
            .next
            .wasm_contract_store(contract_addr, query_msg)
            .await;
        self.observe(
            start,
            Method::WasmQuery,
            if res.is_err() {
                metrics::ResultLabel::Error
            } else {
                metrics::ResultLabel::Success
            },
        )
        .await;

        res
    }
}
