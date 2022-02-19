use std::str::FromStr;

use cosmrs::AccountId;
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Serialize};
use terra_rust_api::Terra;

use crate::MsgExecuteContract;

use crate::Coin;

#[async_trait::async_trait]
pub trait Lcd: Send + Sync {
    async fn bank_balances(&self, account_id: &AccountId) -> anyhow::Result<Vec<Coin>>;

    // FIXME(xla): The endpoint is deprecated, the new one to use is /compute_tax.
    // FIXME(xla): Only works if messages are contract executions.
    async fn estimate_fee(
        &self,
        account_id: &AccountId,
        execute_msgs: Vec<MsgExecuteContract>,
    ) -> anyhow::Result<Decimal256>;

    async fn wasm_query<Q: ?Sized + Sync + Serialize, T: Send + for<'de> Deserialize<'de>>(
        &self,
        contract_addr: &Addr,
        query: &Q,
    ) -> anyhow::Result<T>;
}

pub struct Http {
    terra: Terra,
}

impl Http {
    pub fn new<S: Into<String>>(url: S, chain_id: S) -> Self {
        let mut terra = Terra::lcd_client_no_tx(url, chain_id);
        terra.debug = true;

        Self { terra }
    }
}

#[async_trait::async_trait]
impl Lcd for Http {
    async fn bank_balances(&self, account_id: &AccountId) -> anyhow::Result<Vec<Coin>> {
        let res = self.terra.bank().balances(account_id.as_ref()).await?;
        Ok(res.result.into_iter().map(Into::into).collect())
    }

    async fn estimate_fee(
        &self,
        account_id: &AccountId,
        execute_msgs: Vec<MsgExecuteContract>,
    ) -> anyhow::Result<Decimal256> {
        let mut msgs = vec![];

        for m in &execute_msgs {
            let coins: Vec<terra_rust_api::client::core_types::Coin> =
                m.coins.iter().map(Into::into).collect();
            let msg = terra_rust_api::MsgExecuteContract::create_from_value(
                &account_id.to_string(),
                // "terra1tndcaqxkpc5ce9qee5ggqf430mr2z3pefe5wj6",
                &m.contract_address.to_string(),
                &serde_json::from_slice::<serde_json::Value>(&m.execute_msg)?,
                &coins,
            )?;

            msgs.push(msg);
        }

        let res = self
            .terra
            .tx()
            .estimate_fee(
                &account_id.to_string(),
                &msgs,
                1.0,
                &[&terra_rust_api::client::core_types::Coin {
                    amount: rust_decimal::Decimal::from_f32(0.15).unwrap(),
                    denom: "uusd".to_string(),
                }],
            )
            .await?;

        let uusd = res
            .result
            .fee
            .amount
            .iter()
            .find(|coin| coin.denom == "uusd")
            .unwrap();
        let fee = Decimal256::from_str(&uusd.amount.to_string()).unwrap();

        Ok(fee)
    }

    async fn wasm_query<Q: ?Sized + Sync + Serialize, T: Send + for<'de> Deserialize<'de>>(
        &self,
        contract_address: &Addr,
        query: &Q,
    ) -> anyhow::Result<T> {
        self.terra
            .wasm()
            .query(
                &contract_address.to_string(),
                &serde_json::to_string(query)?,
            )
            .await
    }
}
