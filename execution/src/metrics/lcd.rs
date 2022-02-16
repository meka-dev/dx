use std::{sync::Arc, time::Instant};

use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;
use prometheus_client::encoding::text::Encode;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use terrars::{lcd::Lcd, AccountId, BaseAccount, Coin, MsgExecuteContract};

use crate::metrics;

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    AuthAccount,
    BankBalances,
    EstimateFee,
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

#[derive(Clone)]
pub struct Observed<L>
where
    L: Clone,
{
    next: L,
    metrics: Arc<Mutex<metrics::Metrics>>,
    version: String,
}

impl<L> Observed<L>
where
    L: Clone,
{
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
    L: Clone + Lcd,
{
    async fn auth_account(&self, account_id: &AccountId) -> anyhow::Result<Option<BaseAccount>> {
        let start = Instant::now();
        let res = self.next.auth_account(account_id).await;
        self.observe(
            start,
            Method::AuthAccount,
            if res.is_err() {
                ResultLabel::Error
            } else {
                ResultLabel::Success
            },
        )
        .await;

        res
    }

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

    async fn estimate_fee(
        &self,
        account_id: &AccountId,
        execute_msgs: Vec<MsgExecuteContract>,
    ) -> anyhow::Result<Decimal256> {
        let start = Instant::now();
        let res = self.next.estimate_fee(account_id, execute_msgs).await;
        self.observe(
            start,
            Method::EstimateFee,
            if res.is_err() {
                ResultLabel::Error
            } else {
                ResultLabel::Success
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
    ) -> anyhow::Result<T> {
        let start = Instant::now();
        let res = self
            .next
            .wasm_contract_store(contract_addr, query_msg)
            .await;
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
