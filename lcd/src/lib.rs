use async_trait::async_trait;
use chainrs::cosmos::{
    auth::{BaseAccount, QueryAccountResponse},
    bank::QueryAllBalancesResponse,
    base::{AccountId, Coin},
};
use common::Result;
use reqwest::{Client, ClientBuilder, Url};
use serde::Deserialize;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Clone, Debug, Deserialize)]
pub struct Error {
    pub code: usize,
    pub message: String,
}

#[derive(Debug)]
pub enum Response<T> {
    Error(Error),
    Ok(T),
}

#[async_trait::async_trait]
pub trait Lcd: Send + Sync {
    async fn auth_account(&self, account_id: &AccountId) -> Result<Option<BaseAccount>>;

    async fn bank_balances(&self, account_id: &AccountId) -> Result<Vec<Coin>>;
}

#[derive(Clone)]
pub struct Http {
    base_url: Url,
    client: Client,
}

impl Http {
    pub fn new(url: String) -> Result<Self> {
        let base_url = Url::parse(&url)?;
        let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

        Ok(Self { base_url, client })
    }
}

#[async_trait]
impl Lcd for Http {
    async fn auth_account(&self, account_id: &AccountId) -> Result<Option<BaseAccount>> {
        let res = self
            .client
            .get(
                self.base_url
                    .join(&format!("cosmos/auth/v1beta1/accounts/{account_id}"))?,
            )
            .send()
            .await?
            .json::<QueryAccountResponse>()
            .await?;

        Ok(res.account)
    }

    async fn bank_balances(&self, account_id: &AccountId) -> Result<Vec<Coin>> {
        let res = self
            .client
            .get(
                self.base_url
                    .join(&format!("cosmos/bank/v1beta1/balances/{account_id}"))?,
            )
            .send()
            .await?
            .json::<QueryAllBalancesResponse>()
            .await?;

        // Ok(res.balances.iter().map(|e| e.into()).collect::<Vec<_>>())
        Ok(res.balances)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use chainrs::cosmos::{auth::BaseAccount, base::AccountId};
    use cosmrs::crypto::secp256k1::SigningKey;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock,
        MockServer,
        ResponseTemplate,
    };

    use super::{Http, Lcd};

    #[tokio::test]
    async fn http_auth_account() -> Result<(), Box<dyn std::error::Error>> {
        let account_id = AccountId::from_str("terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302")?;
        let expected_account = BaseAccount {
            account_number: 2932070,
            address: account_id.to_string(),
            pub_key: Some(SigningKey::random().public_key()),
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
    async fn http_bank_balances() -> Result<(), Box<dyn std::error::Error>> {
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
    async fn http_bank_balances_empty() -> Result<(), Box<dyn std::error::Error>> {
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
}
