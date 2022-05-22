use std::time::Instant;

use chainrs::cosmos::{
    auth::BaseAccount,
    base::{AccountId, Coin},
};
use common::{
    metrics::{Registry, ResultLabel, BUCKETS},
    Result,
};
use lcd::{self, Lcd};
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
enum Method {
    AuthAccount,
    BankBalances,
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
}
