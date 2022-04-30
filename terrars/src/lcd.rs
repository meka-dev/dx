use anyhow::Result;
use async_trait::async_trait;
use cosmrs::{
    auth::{BaseAccount, QueryAccountReponse},
    AccountId,
};
use cosmwasm_std::Addr;
use reqwest::{Client, ClientBuilder, StatusCode, Url};
use serde::{Deserialize, Serialize};

use crate::{bank::QueryAllBalancesResponse, base, tx, wasm::QueryContractStoreResponse};

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

    async fn bank_balances(&self, account_id: &AccountId) -> Result<Vec<base::Coin>>;

    async fn tx_compute_tax(&self, tx: &[u8]) -> Result<Response<Vec<base::Coin>>>;
    async fn tx_simulate(&self, tx: &[u8]) -> Result<Response<tx::SimulateResponse>>;

    async fn wasm_contract_store<
        Q: ?Sized + Sync + Serialize,
        T: Send + for<'de> Deserialize<'de>,
    >(
        &self,
        contract_addr: &Addr,
        query_msg: &Q,
    ) -> Result<T>;
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
                    .join(&format!("cosmos/auth/v1beta1/accounts/{}", account_id))?,
            )
            .send()
            .await?
            .json::<QueryAccountReponse>()
            .await?;

        Ok(res.account)
    }

    async fn bank_balances(&self, account_id: &AccountId) -> Result<Vec<base::Coin>> {
        let res = self
            .client
            .get(
                self.base_url
                    .join(&format!("cosmos/bank/v1beta1/balances/{}", account_id))?,
            )
            .send()
            .await?
            .json::<QueryAllBalancesResponse>()
            .await?;

        Ok(res.balances)
    }

    async fn tx_compute_tax(&self, tx: &[u8]) -> Result<Response<Vec<base::Coin>>> {
        let raw = self
            .client
            .post(self.base_url.join("terra/tx/v1beta1/compute_tax")?)
            .json(&tx::ComputeTaxRequest {
                tx_bytes: base64::encode(tx),
            })
            .send()
            .await?;

        let res = match raw.status() {
            StatusCode::OK => Response::Ok(raw.json::<tx::ComputeTaxResponse>().await?.tax_amount),
            StatusCode::BAD_REQUEST => Response::Error(raw.json::<Error>().await?),
            code => return Err(anyhow::anyhow!("unexpected response status code: {}", code)),
        };

        Ok(res)
    }

    async fn tx_simulate(&self, tx: &[u8]) -> Result<Response<tx::SimulateResponse>> {
        let raw = self
            .client
            .post(self.base_url.join("cosmos/tx/v1beta1/simulate")?)
            .json(&tx::SimulateRequest {
                tx_bytes: base64::encode(tx),
            })
            .send()
            .await?;

        let res = match raw.status() {
            StatusCode::OK => Response::Ok(raw.json::<tx::SimulateResponse>().await?),
            StatusCode::BAD_REQUEST => Response::Error(raw.json::<Error>().await?),
            code => return Err(anyhow::anyhow!("unexpected response status code: {}", code)),
        };

        Ok(res)
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

    use cosmrs::{
        tendermint::{block::Height, chain},
        tx::Msg as _,
    };
    use cosmwasm_bignumber::Uint256;
    use cosmwasm_std::Addr;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock,
        MockServer,
        ResponseTemplate,
    };

    use super::{Http, Lcd, Response};
    use crate::{base::Coin, tx, AccountId, BaseAccount, SigningKey};

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

    #[tokio::test]
    async fn http_tx_simulate() -> Result<(), Box<dyn std::error::Error>> {
        let account_id = AccountId::from_str("terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l")?;
        let private_key = SigningKey::random();
        let bank_account = BaseAccount {
            account_number: 0,
            address: account_id.to_string(),
            pub_key: Some(private_key.public_key()),
            sequence: 0,
        };
        let tx = {
            let send = cosmrs::bank::MsgSend {
                amount: vec![Coin {
                    amount: Uint256::one(),
                    denom: cosmrs::Denom::from_str("uusd")?,
                }
                .into()],
                from_address: AccountId::from_str("terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l")?,
                to_address: AccountId::from_str("terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302")?,
            };

            tx::sign(
                chain::Id::from_str("columbus-5")?,
                &private_key,
                bank_account.account_number,
                bank_account.sequence,
                1000,
                1000,
                None,
                None,
                Height::try_from(0u64)?,
                vec![send.to_any()?],
            )?
        };

        let server = MockServer::start().await;
        let response = ResponseTemplate::new(200).set_body_json(&json!({
            "gas_info": {
                "gas_wanted": "0",
                "gas_used": "66467"
            },
            "result": {
                "data": "Ch4KHC9jb3Ntb3MuYmFuay52MWJldGExLk1zZ1NlbmQ=",
                "log": "[{\"events\":[{\"type\":\"coin_received\",\"attributes\":[{\"key\":\"receiver\",\"value\":\"terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302\"},{\"key\":\"amount\",\"value\":\"1000000uusd\"}]},{\"type\":\"coin_spent\",\"attributes\":[{\"key\":\"spender\",\"value\":\"terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l\"},{\"key\":\"amount\",\"value\":\"1000000uusd\"}]},{\"type\":\"message\",\"attributes\":[{\"key\":\"action\",\"value\":\"/cosmos.bank.v1beta1.MsgSend\"},{\"key\":\"sender\",\"value\":\"terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l\"},{\"key\":\"module\",\"value\":\"bank\"}]},{\"type\":\"transfer\",\"attributes\":[{\"key\":\"recipient\",\"value\":\"terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302\"},{\"key\":\"sender\",\"value\":\"terra175yp3rnjk2av64zlkt777kkqa7f829g2hfpn4l\"},{\"key\":\"amount\",\"value\":\"1000000uusd\"}]}]}]",
                "events": [
                {
                    "type": "message",
                    "attributes": [
                    {
                        "key": "YWN0aW9u",
                        "value": "L2Nvc21vcy5iYW5rLnYxYmV0YTEuTXNnU2VuZA==",
                        "index": false
                    }
                    ]
                },
                {
                    "type": "coin_spent",
                    "attributes": [
                    {
                        "key": "c3BlbmRlcg==",
                        "value": "dGVycmExNzV5cDNybmprMmF2NjR6bGt0Nzc3a2txYTdmODI5ZzJoZnBuNGw=",
                        "index": false
                    },
                    {
                        "key": "YW1vdW50",
                        "value": "MTAwMDAwMHV1c2Q=",
                        "index": false
                    }
                    ]
                },
                {
                    "type": "coin_received",
                    "attributes": [
                    {
                        "key": "cmVjZWl2ZXI=",
                        "value": "dGVycmExZW1sN2czbGw2amt5aHRmdjJnMGd2cW56enB5NmtqeWQ3cXIzMDI=",
                        "index": false
                    },
                    {
                        "key": "YW1vdW50",
                        "value": "MTAwMDAwMHV1c2Q=",
                        "index": false
                    }
                    ]
                },
                {
                    "type": "transfer",
                    "attributes": [
                    {
                        "key": "cmVjaXBpZW50",
                        "value": "dGVycmExZW1sN2czbGw2amt5aHRmdjJnMGd2cW56enB5NmtqeWQ3cXIzMDI=",
                        "index": false
                    },
                    {
                        "key": "c2VuZGVy",
                        "value": "dGVycmExNzV5cDNybmprMmF2NjR6bGt0Nzc3a2txYTdmODI5ZzJoZnBuNGw=",
                        "index": false
                    },
                    {
                        "key": "YW1vdW50",
                        "value": "MTAwMDAwMHV1c2Q=",
                        "index": false
                    }
                    ]
                },
                {
                    "type": "message",
                    "attributes": [
                    {
                        "key": "c2VuZGVy",
                        "value": "dGVycmExNzV5cDNybmprMmF2NjR6bGt0Nzc3a2txYTdmODI5ZzJoZnBuNGw=",
                        "index": false
                    }
                    ]
                },
                {
                    "type": "message",
                    "attributes": [
                    {
                        "key": "bW9kdWxl",
                        "value": "YmFuaw==",
                        "index": false
                    }
                    ]
                }
                ]
            }
            }));
        Mock::given(method("POST"))
            .and(path("/cosmos/tx/v1beta1/simulate"))
            .respond_with(response)
            .mount(&server)
            .await;

        let lcd = Http::new(server.uri())?;
        let res = lcd.tx_simulate(&tx).await?;

        matches!(res, Response::Ok(_));

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
            .and(path(
                "/terra/wasm/v1beta1/contracts/terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g/store",
            ))
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
