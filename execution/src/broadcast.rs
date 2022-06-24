use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

use arby::msg::ExecuteMsg;
use async_trait::async_trait;
use chainrs::{
    cosmos::{
        auth::BaseAccount,
        base::{AccountId, Coin},
    },
    osmosis::gamm::{MsgSwapExactAmountIn, SwapAmountInRoute},
    proto::cosmos::base::abci::v1beta1::GasInfo,
    tx::{self, Msg},
};
use common::{
    err,
    metrics::{Registry, ResultLabel, BUCKETS},
    tracing::{debug, error, info},
    Context as _,
    Result,
};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmwasm_std::{Addr, Uint128};
use itertools::Itertools;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Unit,
};
use strategy::{PoolId, Trade};
use tendermint::{abci::Transaction, block::Height, chain};
use tendermint_rpc::{endpoint::broadcast, Client as _, WebSocketClient};
use terrars::{dex::Token, wasm::MsgExecuteContract};
use tokio::{
    sync::{mpsc, Mutex},
    time::Instant,
};

use crate::{accounts, tracker::Tracker};

pub type Fee = u64;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Profitable,
    NotProfitable,
}

#[derive(Debug)]
pub struct Simulation {
    fee: u64,
    gas_info: GasInfo,
    outcome: Outcome,
}

pub async fn run<B, T>(
    mut trades: mpsc::Receiver<Trade>,
    mut broadcaster: B,
    tx_tracker: Arc<Mutex<T>>,
    execute_trades: Arc<RwLock<bool>>,
) -> Result<()>
where
    B: Broadcaster,
    T: Tracker,
{
    debug!("Init broadcaster");

    while let Some(trade) = trades.recv().await {
        if let Some(addr) = (*tx_tracker.lock().await).is_tracked(&trade.route) {
            debug!("pool {} in route  ", addr);
            continue;
        }

        if let Ok(simulation) = broadcaster.simulate(&trade).await {
            let execute_trades = Arc::clone(&execute_trades);
            let execute_trades =
                tokio::task::spawn_blocking(move || *execute_trades.read().unwrap())
                    .await
                    .unwrap();

            if execute_trades && simulation.outcome == Outcome::Profitable {
                if let Ok(check) = broadcaster
                    .broadcast(simulation.fee, simulation.gas_info, &trade)
                    .await
                {
                    if check.code.is_ok() {
                        let _ = (*tx_tracker.lock().await).track(check.hash, &trade.route);
                    }
                }
            }
        }
    }

    Ok(())
}

#[async_trait]
pub trait Broadcaster {
    async fn broadcast(
        &mut self,
        fee: Fee,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response>;

    async fn simulate(&self, trade: &Trade) -> Result<Simulation>;
}

#[allow(dead_code)]
pub struct Rpc {
    account: BaseAccount,
    client: WebSocketClient,
    chain_id: chain::Id,
    signing_key_bytes: Vec<u8>,
    sequence: u64,
    treasury: Option<AccountId>,
}

impl Rpc {
    pub fn new(
        client: WebSocketClient,
        chain_id: chain::Id,
        account: BaseAccount,
        signing_key_bytes: Vec<u8>,
        treasury: Option<AccountId>,
    ) -> Self {
        let sequence = account.sequence;

        Self {
            account,
            chain_id,
            client,
            signing_key_bytes,
            sequence,
            treasury,
        }
    }
}

#[allow(dead_code)]
fn mk_msg_terra(
    sender: &AccountId,
    proxy_addr: &AccountId,
    t: &Trade,
) -> Result<MsgExecuteContract> {
    let path = t
        .legs
        .iter()
        .map(|leg| {
            (
                Addr::unchecked(leg.pool.id.clone()),
                Into::<Token>::into(leg.pool.ask_asset(&leg.out_token).unwrap()).into(),
            )
        })
        .collect_vec();
    Ok(MsgExecuteContract {
        coins: vec![],
        contract_address: Addr::unchecked(proxy_addr.to_string()),
        execute_msg: serde_json::to_vec(&ExecuteMsg::Sprint {
            input: t.size_to_int().0.as_u64().into(),
            min_out: Uint128::new(0u128),
            path,
        })?,
        sender: sender.to_string(),
    })
}

fn mk_msg_osmosis(sender: &AccountId, t: &Trade) -> Result<MsgSwapExactAmountIn> {
    let min_out = t.size_to_int();
    let path = t
        .legs
        .iter()
        .map(|leg| SwapAmountInRoute {
            pool_id: leg.pool.id.clone(),
            token_out_denom: leg.out_token.id.clone(),
        })
        .collect_vec();
    Ok(MsgSwapExactAmountIn {
        sender: sender.to_string(),
        routes: path,
        token_in: Some(Coin {
            denom: chainrs::cosmos::base::Denom::from_str(&t.in_token.id)?,
            amount: t
                .size_to_int()
                .0
                .as_u64() // XXX
                .into(),
        }),
        token_out_min_amount: min_out.to_string(),
    })
}

#[async_trait]
impl Broadcaster for Rpc {
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response> {
        let ai = AccountId::from_str(
            SigningKey::from_bytes(&self.signing_key_bytes)
                .map_err(|err| err!("failed to craft signing key from bytes: {}", err))?
                .public_key()
                .account_id(accounts::PREFIX)
                .unwrap()
                .as_ref(),
        )?;
        let msg = mk_msg_osmosis(&ai, trade)?.to_any()?;
        let tx = tx::sign(
            self.chain_id.clone(),
            &SigningKey::from_bytes(&self.signing_key_bytes)
                .map_err(|err| err!("failed to craft signing key from bytes: {}", err))?,
            self.account.account_number,
            self.sequence,
            // NB(xla): Using fixed gas prices here, could diverge.
            fee,
            "uosmo".to_string(),
            (gas_info.gas_used as f64 * 1.10) as u64,
            None,
            None,
            Height::try_from(0_u64).unwrap(),
            vec![msg],
        )
        .context("failed to sign transaction")?;

        let check = self
            .client
            .broadcast_tx_sync(Transaction::from(tx))
            .await
            .context("failed to broadcast tx")?;
        // Advance sequence if tx succeeded.
        if check.code.is_ok() {
            self.sequence += 1;
        }

        Ok(check)
    }

    async fn simulate(&self, trade: &Trade) -> Result<Simulation> {
        let expected_profit: u128 = trade.expected_profit_to_int().into();
        let gas_used = trade
            .route
            .iter()
            .fold(0u64, |acc, m| acc + gas_estimator(m));

        let fee = (gas_used as f64 * 0.007f64) as u64;
        let outcome = if expected_profit > fee.into() {
            Outcome::Profitable
        } else {
            Outcome::NotProfitable
        };
        Ok(Simulation {
            fee,
            gas_info: GasInfo {
                gas_wanted: 0,
                gas_used,
            },
            outcome,
        })
    }
}

// NB(pm): needs rework
fn gas_estimator(_addr: &PoolId) -> u64 {
    85000
}

impl Labeled for Rpc {
    const NAME: Name = Name::Rpc;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Name {
    Rpc,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Method {
    Broadcast,
    Simulate,
}

pub trait Labeled {
    const NAME: Name;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
struct CallLabels {
    name: Name,
    method: Method,
    result: ResultLabel,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
struct PerformanceLabels {
    name: Name,
    method: Method,
}

pub struct Observed<N> {
    next: N,

    call_duration: Family<CallLabels, Histogram>,
    estimated_fee_uusd: Family<PerformanceLabels, Counter>,
    expected_profit_after_fee_uusd: Family<PerformanceLabels, Counter>,
}

impl<N> Observed<N> {
    pub fn new(registry: &mut Registry, next: N) -> Self {
        let call_duration = Family::<CallLabels, Histogram>::new_with_constructor(|| {
            Histogram::new(BUCKETS.clone().into_iter())
        });
        registry.register_with_unit(
            "broadcast_call_duration",
            "Broacast call durations",
            Unit::Seconds,
            Box::new(call_duration.clone()),
        );
        let estimated_fee_uusd = Family::<PerformanceLabels, Counter>::default();
        registry.register(
            "broadast_estimated_fee_uusd",
            "Estimated transaction fee",
            Box::new(estimated_fee_uusd.clone()),
        );
        let expected_profit_after_fee_uusd = Family::<PerformanceLabels, Counter>::default();
        registry.register(
            "broadcast_expected_profit_after_fee_uusd",
            "Expected profit after fees have been subtracted",
            Box::new(expected_profit_after_fee_uusd.clone()),
        );

        Self {
            next,

            call_duration,
            estimated_fee_uusd,
            expected_profit_after_fee_uusd,
        }
    }
}

impl<N> Observed<N>
where
    N: Labeled,
{
    fn observe(&self, method: Method, start: Instant, is_err: bool) {
        self.call_duration
            .get_or_create(&CallLabels {
                name: N::NAME,
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

#[async_trait]
impl<N> Broadcaster for Observed<N>
where
    N: Broadcaster + Labeled + Send + Sync + 'static,
{
    async fn broadcast(
        &mut self,
        fee: u64,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response> {
        let expected_profit = trade.expected_profit_to_int().0.as_u64();
        let method = Method::Broadcast;
        let start = Instant::now();
        let res = self.next.broadcast(fee, gas_info, trade).await;

        self.observe(method.clone(), start, res.is_err());

        let perf_labels = PerformanceLabels {
            name: N::NAME,
            method,
        };
        self.estimated_fee_uusd
            .get_or_create(&perf_labels)
            .inc_by(fee);

        if fee < expected_profit {
            self.expected_profit_after_fee_uusd
                .get_or_create(&perf_labels)
                .inc_by(expected_profit - fee);
        }

        match res {
            Ok(check) => {
                info!(
                    "msg=submit hash={} code={:?} log={}",
                    check.hash, check.code, check.log
                );

                Ok(check)
            },
            Err(err) => {
                error!("failed to submit tx: {}", err);
                Err(err)
            },
        }
    }

    async fn simulate(&self, trade: &Trade) -> Result<Simulation> {
        let start = Instant::now();
        let res = self.next.simulate(trade).await;

        self.observe(Method::Simulate, start, res.is_err());

        match res {
            Ok(simulation) => {
                info!(
                    "legs={} expected_profit={} estimated_fee={} outcome={:?}",
                    trade.legs.len(),
                    trade.expected_profit,
                    simulation.fee,
                    simulation.outcome,
                );

                Ok(simulation)
            },
            Err(err) => {
                error!("failed to simulate tx: {}", err);
                Err(err)
            },
        }
    }
}
