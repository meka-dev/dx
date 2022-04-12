use std::fmt;

use anyhow::Result;
use common::{
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error},
};
use cosmwasm_std::Addr;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, histogram::Histogram},
    registry::Unit,
};
use tendermint::block::Height;
use terrars::{dex, lcd::Lcd, mekapi, AccountId};
use tokio::{task::JoinHandle, time::Instant};

use crate::accounts;

#[async_trait::async_trait]
pub trait Fetcher<L, M>
where
    L: Lcd,
    M: mekapi::v3::Api,
{
    async fn fetch(
        &self,
        lcd: L,
        mekapi: M,
        account_id: &AccountId,
        height: Height,
    ) -> Result<(accounts::Balances, Vec<dex::Pool>)>;
}

pub struct Mekapi {}

impl Labeled for Mekapi {
    const NAME: Name = Name::Mekapi;
}

#[async_trait::async_trait]
impl<L, M> Fetcher<L, M> for Mekapi
where
    L: Lcd + 'static,
    M: mekapi::v3::Api + Clone + 'static,
{
    async fn fetch(
        &self,
        lcd: L,
        mekapi: M,
        account_id: &AccountId,
        height: Height,
    ) -> Result<(accounts::Balances, Vec<dex::Pool>)> {
        // Fetch available funds for strategies to incorporate.
        let funds_handle = tokio::spawn(accounts::fetch(lcd, account_id.clone()));
        let astroport_handle = {
            let mekapi = mekapi.clone();
            tokio::spawn(async move {
                mekapi
                    .v3_pools(Addr::unchecked(terrars::dex::astroport::ADDRESS), height)
                    .await
            })
        };
        let loopmarkets_handle = {
            let mekapi = mekapi.clone();
            tokio::spawn(async move {
                mekapi
                    .v3_pools(Addr::unchecked(terrars::dex::loopmarkets::ADDRESS), height)
                    .await
            })
        };
        let terraswap_handle = {
            let mekapi = mekapi.clone();
            tokio::spawn(async move {
                mekapi
                    .v3_pools(Addr::unchecked(terrars::dex::terraswap::ADDRESS), height)
                    .await
            })
        };
        let (funds, astroport, mut loopmarkets, mut terraswap) = tokio::try_join!(
            flatten(funds_handle),
            flatten(astroport_handle),
            flatten(loopmarkets_handle),
            flatten(terraswap_handle)
        )?;

        let mut pools = astroport;
        pools.append(&mut loopmarkets);
        pools.append(&mut terraswap);

        Ok((
            funds,
            pools.into_iter().map(dex::Pool::from).collect::<Vec<_>>(),
        ))
    }
}

async fn flatten<T>(handle: JoinHandle<Result<T>>) -> Result<T> {
    match handle.await {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(anyhow::anyhow!("handle failed: {}", err)),
    }
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    Mekapi,
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Mekapi => write!(f, "mekapi"),
        }
    }
}

pub trait Labeled {
    const NAME: Name;

    fn name(&self) -> Name {
        Self::NAME
    }
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    name: Name,
    result: ResultLabel,
}

pub struct Observed<F> {
    call_duration: Family<Labels, Histogram>,
    next: F,
}

impl<F> Observed<F> {
    pub fn new(registry: &mut Registry, next: F) -> Self {
        let call_duration = Family::<Labels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });

        registry.register_with_unit(
            "pool_fetcher_call_duration",
            "Fetcher pools retrieval durations",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );

        Self {
            call_duration,
            next,
        }
    }
}

#[async_trait::async_trait]
impl<L, M, F> Fetcher<L, M> for Observed<F>
where
    L: Lcd + 'static,
    M: mekapi::v3::Api + Clone + 'static,
    F: Fetcher<L, M> + Labeled + Send + Sync,
{
    async fn fetch(
        &self,
        lcd: L,
        mekapi: M,
        account_id: &AccountId,
        height: Height,
    ) -> Result<(accounts::Balances, Vec<dex::Pool>)> {
        let start = Instant::now();
        let res = self.next.fetch(lcd, mekapi, account_id, height).await;
        let elapsed = start.elapsed().as_secs_f64();

        self.call_duration
            .get_or_create(&Labels {
                name: self.next.name(),
                result: if res.is_err() {
                    ResultLabel::Error
                } else {
                    ResultLabel::Success
                },
            })
            .observe(elapsed);

        match res {
            Ok(_) => debug!(
                target: "fetcher",
                "fetcher={} elapsed={}",
                self.next.name(),
                elapsed,
            ),
            Err(ref err) => error!(
                target: "fetcher",
                "fetcher={} elapsed={} err={}",
                self.next.name(),
                elapsed,
                err
            ),
        }

        res
    }
}
