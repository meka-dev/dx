use std::str::FromStr;

use cosmrs::{
    auth::{BaseAccount, QueryAccountReponse},
    AccountId,
};
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;
use reqwest::{Client, ClientBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::{
    bank::QueryAllBalancesResponse, wasm::QueryContractStoreResponse, Coin, MsgExecuteContract,
};

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[async_trait::async_trait]
pub trait Lcd: Send + Sync {
    async fn auth_account(&self, account_id: &AccountId) -> anyhow::Result<Option<BaseAccount>>;

    async fn bank_balances(&self, account_id: &AccountId) -> anyhow::Result<Vec<Coin>>;

    // FIXME(xla): The endpoint is deprecated, the new one to use is /compute_tax.
    // FIXME(xla): Only works if messages are contract executions.
    async fn estimate_fee(
        &self,
        account_id: &AccountId,
        execute_msgs: Vec<MsgExecuteContract>,
    ) -> anyhow::Result<Decimal256>;

    async fn wasm_contract_store<
        Q: ?Sized + Sync + Serialize,
        T: Send + for<'de> Deserialize<'de>,
    >(
        &self,
        contract_addr: &Addr,
        query_msg: &Q,
    ) -> anyhow::Result<T>;
}

#[derive(Clone)]
pub struct Http {
    base_url: Url,
    client: Client,
}

impl Http {
    pub fn new(url: String) -> anyhow::Result<Self> {
        let base_url = Url::parse(&url)?;
        let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

        Ok(Self { base_url, client })
    }
}

#[async_trait::async_trait]
impl Lcd for Http {
    async fn auth_account(&self, account_id: &AccountId) -> anyhow::Result<Option<BaseAccount>> {
        let res = self
            .client
            .get(
                self.base_url
                    .join(&format!("cosmos/auth/v1beta1/accounts/{}", account_id,))?,
            )
            .send()
            .await?
            .json::<QueryAccountReponse>()
            .await?;

        Ok(res.account)
    }

    async fn bank_balances(&self, account_id: &AccountId) -> anyhow::Result<Vec<Coin>> {
        let res = self
            .client
            .get(
                self.base_url
                    .join(&format!("cosmos/bank/v1beta1/balances/{}", account_id,))?,
            )
            .send()
            .await?
            .json::<QueryAllBalancesResponse>()
            .await?;

        Ok(res.balances)
    }

    // FIXME(xla): Short-circuit this as it will be superseded by compute_tax.
    async fn estimate_fee(
        &self,
        _account_id: &AccountId,
        _execute_msgs: Vec<MsgExecuteContract>,
    ) -> anyhow::Result<Decimal256> {
        Ok(Decimal256::from_str("25000")?)
    }

    async fn wasm_contract_store<
        Q: ?Sized + Sync + Serialize,
        T: Send + for<'de> Deserialize<'de>,
    >(
        &self,
        contract_address: &Addr,
        query_msg: &Q,
    ) -> anyhow::Result<T> {
        let mut url = self.base_url.join(&format!(
            "terra/wasm/v1beta1/contracts/{}/store",
            contract_address,
        ))?;
        url.query_pairs_mut().clear().append_pair(
            "query_msg",
            &base64::encode_config(&serde_json::to_vec(query_msg)?, base64::URL_SAFE),
        );

        let res = self
            .client
            .get(url)
            .send()
            .await?
            .json::<QueryContractStoreResponse<T>>()
            .await?;

        Ok(res.query_result)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use cosmwasm_std::Addr;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    use crate::{AccountId, BaseAccount, SigningKey};

    use super::{Http, Lcd};

    #[tokio::test]
    async fn http_auth_account() -> Result<(), Box<dyn std::error::Error>> {
        let account_id = AccountId::from_str("terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302")?;
        let expected_account = BaseAccount {
            account_number: 2932070,
            address: account_id.to_string(),
            pub_key: SigningKey::random().public_key(),
            sequence: 6,
        };

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(&json!({
            "account": Some(expected_account.clone()),
        }));
        Mock::given(method("GET"))
            .and(path(
                "/cosmos/auth/v1beta1/accounts/terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302",
            ))
            .respond_with(response)
            .mount(&server)
            .await;

        let lcd = Http::new(server.uri())?;
        let account = lcd.auth_account(&account_id).await?.unwrap();

        matches!(
            account,
            BaseAccount {
                account_number,
                address,
                sequence,
                ..
            } if account_number == expected_account.account_number && address == expected_account.address && sequence == expected_account.sequence
        );

        Ok(())
    }

    #[tokio::test]
    async fn http_auth_account_missing() -> Result<(), Box<dyn std::error::Error>> {
        let account_id = AccountId::from_str("terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l")?;

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(404).set_body_json(&json!({
            "code": 5,
            "message": "rpc error: code = NotFound desc = account terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l not found: key not found",
            "details": []
          }));
        Mock::given(method("GET"))
            .and(path(
                "/cosmos/auth/v1beta1/accounts/terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l",
            ))
            .respond_with(response)
            .mount(&server)
            .await;

        let lcd = Http::new(server.uri())?;
        let account = lcd.auth_account(&account_id).await?;

        assert!(account.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn http_auth_bank_balances() -> Result<(), Box<dyn std::error::Error>> {
        let account_id = AccountId::from_str("terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302")?;

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(&json!({
          "balances": [
            {
              "denom": "uluna",
              "amount": "690751"
            },
            {
              "denom": "uusd",
              "amount": "2622376"
            }
          ],
          "pagination": {
            "next_key": null,
            "total": "19"
          }
        }));
        Mock::given(method("GET"))
            .and(path(
                "/cosmos/bank/v1beta1/balances/terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302",
            ))
            .respond_with(response)
            .mount(&server)
            .await;

        let lcd = Http::new(server.uri())?;
        let coins = lcd.bank_balances(&account_id).await?;

        assert_eq!(coins.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn http_auth_bank_balances_empty() -> Result<(), Box<dyn std::error::Error>> {
        let account_id = AccountId::from_str("terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l")?;

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(&json!({
          "balances": [],
          "pagination": {
            "next_key": null,
            "total": "0"
          }
        }));
        Mock::given(method("GET"))
            .and(path(
                "/cosmos/bank/v1beta1/balances/terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l",
            ))
            .respond_with(response)
            .mount(&server)
            .await;

        let lcd = Http::new(server.uri())?;
        let coins = lcd.bank_balances(&account_id).await?;

        assert_eq!(coins.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn http_query_wasm() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize, Serialize)]
        enum Query {
            Ask {},
        }

        #[derive(Deserialize, Serialize)]
        struct Response {
            answer: String,
        }

        let contract_address = Addr::unchecked("terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g");

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(&json!({
          "query_result": {
              "answer": "yes"
          },
        }));
        Mock::given(method("GET"))
            .and(path(format!(
                "/terra/wasm/v1beta1/contracts/terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g/store"
            )))
            .and(query_param(
                "query_msg",
                base64::encode_config(serde_json::to_vec(&Query::Ask {})?, base64::URL_SAFE),
            ))
            .respond_with(response)
            .mount(&server)
            .await;

        let lcd = Http::new(server.uri())?;
        let res = lcd
            .wasm_contract_store::<Query, Response>(&contract_address, &Query::Ask {})
            .await?;

        assert_eq!(res.answer, "yes".to_string());

        Ok(())
    }
}
