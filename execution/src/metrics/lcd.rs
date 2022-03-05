use std::time::Instant;

use anyhow::Result;
use cosmwasm_std::Addr;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use serde::{Deserialize, Serialize};

use common::metrics::{Registry, ResultLabel, BUCKETS};
use terrars::{
    base::Coin,
    lcd::{self, Lcd, Response},
    tx, AccountId, BaseAccount,
};

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
enum Method {
    AuthAccount,
    BankBalances,
    TxComputeTax,
    TxSimulate,
    WasmQuery,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
struct Labels {
    method: Method,
    result: ResultLabel,
}

#[derive(Clone)]
pub struct Observed<L>
where
    L: Clone,
{
    call_duration: Family<Labels, Histogram>,
    next: L,
}

impl<L> Observed<L>
where
    L: Clone,
{
    pub fn new(registry: &mut Registry, next: L) -> Self {
        let call_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "lcd_call_duration",
            "LCD call latency",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );

        Self {
            call_duration,
            next,
        }
    }

    async fn observe(&self, start: Instant, method: Method, is_err: bool) {
        self.call_duration
            .get_or_create(&Labels {
                method,
                result: if is_err {
                    ResultLabel::Error
                } else {
                    ResultLabel::Success
                },
            })
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
        self.observe(start, Method::AuthAccount, res.is_err()).await;

        res
    }

    async fn bank_balances(&self, account_id: &AccountId) -> Result<Vec<Coin>> {
        let start = Instant::now();
        let res = self.next.bank_balances(account_id).await;
        self.observe(start, Method::BankBalances, res.is_err())
            .await;

        res
    }

    async fn tx_compute_tax(&self, tx: &[u8]) -> Result<lcd::Response<Vec<Coin>>> {
        let start = Instant::now();
        let res = self.next.tx_compute_tax(tx).await;
        self.observe(start, Method::TxComputeTax, res.is_err())
            .await;

        res
    }

    async fn tx_simulate(&self, tx: &[u8]) -> Result<Response<tx::SimulateResponse>> {
        let start = Instant::now();
        let res = self.next.tx_simulate(tx).await;
        self.observe(start, Method::TxSimulate, res.is_err()).await;

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
        let is_err = res.is_err();
        self.observe(start, Method::WasmQuery, is_err).await;

        res
    }
}
