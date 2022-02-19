use cosmrs::AccountId;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use terra_rust_api::Terra;

use crate::Coin;

#[async_trait::async_trait]
pub trait Lcd: Send + Sync {
    async fn bank_balances(&self, account_id: &AccountId) -> anyhow::Result<Vec<Coin>>;

    async fn wasm_query<Q: ?Sized + Sync + Serialize, T: for<'de> Deserialize<'de>>(
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

    async fn wasm_query<Q: ?Sized + Sync + Serialize, T: for<'de> Deserialize<'de>>(
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
