use anyhow::Result;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{counter::Counter, family::Family},
};
use tendermint::{block::Height, chain};

use common::{
    metrics::Registry,
    tracing::{error, info},
};
use terrars::{base::abci::GasInfo, lcd, tx, Message};

#[async_trait::async_trait]
pub trait Simulator {
    async fn simulate(
        &self,
        secret_key_bytes: &[u8],
        chain_id: chain::Id,
        account_number: u64,
        sequence: u64,
        messages: Vec<Message>,
        expected_profit: u64,
    ) -> Result<(u64, GasInfo)>;
}

pub struct Rpc<L> {
    lcd: L,
}

impl<L> Rpc<L> {
    pub fn new(lcd: L) -> Self {
        Self { lcd }
    }
}

// TODO(xla): Fetch gas prices.
#[async_trait::async_trait]
impl<L> Simulator for Rpc<L>
where
    L: lcd::Lcd,
{
    async fn simulate(
        &self,
        secret_key_bytes: &[u8],
        chain_id: chain::Id,
        account_number: u64,
        sequence: u64,
        messages: Vec<Message>,
        _expected_profit: u64,
    ) -> Result<(u64, GasInfo)> {
        let simulate_tx = tx::sign(
            chain_id,
            // FIXME(xla): Proper error handling.
            &terrars::SigningKey::from_bytes(secret_key_bytes).unwrap(),
            account_number,
            sequence,
            0,
            0,
            Height::try_from(0_u64).unwrap(),
            messages,
        )?;

        match self.lcd.tx_simulate(&simulate_tx).await? {
            lcd::Response::Ok(tx::SimulateResponse { gas_info, .. }) => {
                let fee = (gas_info.gas_used as f64 * 0.18) as u64;
                Ok((fee, gas_info))
            },
            lcd::Response::Error(lcd::Error { message, .. }) => {
                Err(anyhow::anyhow!("tx simulation failed: {}", message))
            },
        }
    }
}

impl<L> Labeled for Rpc<L> {
    const NAME: Name = Name::Rpc;
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
enum Name {
    Rpc,
}

trait Labeled {
    const NAME: Name;

    fn name(&self) -> Name {
        Self::NAME
    }
}

#[derive(Clone, Copy, Debug, Hash, Encode, Eq, PartialEq)]
enum Outcome {
    Profitable,
    NotProfitable,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct Labels {
    name: Name,
    outcome: Outcome,
}

#[derive(Clone, Hash, Encode, Eq, PartialEq)]
pub struct FailureLabels {
    name: Name,
}

pub struct Observed<S> {
    estimated_fee_uusd: Family<Labels, Counter>,
    expected_profit_after_fee_uusd: Family<Labels, Counter>,
    failures: Family<FailureLabels, Counter>,
    next: S,
}

impl<S> Observed<S> {
    pub fn new(registry: &mut Registry, next: S) -> Self {
        let estimated_fee_uusd = Family::<Labels, Counter>::default();
        registry.register(
            "simulate_estimated_fee_uusd",
            "Estimated transaction fee",
            Box::new(estimated_fee_uusd.clone()),
        );
        let expected_profit_after_fee_uusd = Family::<Labels, Counter>::default();
        registry.register(
            "simulate_expected_profit_after_fee_uusd",
            "Expected profit after fees have been subtracted",
            Box::new(expected_profit_after_fee_uusd.clone()),
        );
        let failures = Family::<FailureLabels, Counter>::default();

        Self {
            estimated_fee_uusd,
            expected_profit_after_fee_uusd,
            failures,
            next,
        }
    }
}

#[async_trait::async_trait]
impl<S> Simulator for Observed<S>
where
    S: Simulator + Labeled + Send + Sync,
{
    async fn simulate(
        &self,
        secret_key_bytes: &[u8],
        chain_id: chain::Id,
        account_number: u64,
        sequence: u64,
        messages: Vec<Message>,
        expected_profit: u64,
    ) -> Result<(u64, GasInfo)> {
        let messages_len = messages.len();
        let res = self
            .next
            .simulate(
                secret_key_bytes,
                chain_id,
                account_number,
                sequence,
                messages,
                expected_profit,
            )
            .await;

        match res {
            Ok((fee, gas_info)) => {
                let outcome = if fee <= expected_profit {
                    Outcome::Profitable
                } else {
                    Outcome::NotProfitable
                };

                let labels = Labels {
                    name: self.next.name(),
                    outcome,
                };
                self.estimated_fee_uusd.get_or_create(&labels).inc_by(fee);

                if fee <= expected_profit {
                    self.expected_profit_after_fee_uusd
                        .get_or_create(&labels)
                        .inc_by(expected_profit - fee);
                }

                info!(
                    "messages={} expected_profit={} estimated_fee={} outcome={:?}",
                    messages_len, expected_profit, fee, outcome,
                );

                Ok((fee, gas_info))
            },
            Err(err) => {
                self.failures
                    .get_or_create(&FailureLabels {
                        name: self.next.name(),
                    })
                    .inc();
                error!("failed to simulate tx: {}", err);
                Err(err)
            },
        }
    }
}
