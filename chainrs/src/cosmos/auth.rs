//! Auth module supprot
//!
//! <https://docs.cosmos.network/master/modules/bank/>

use std::str::FromStr;

use common::{Report, Result};
use cosmrs::crypto::PublicKey;
use serde::{Deserialize, Serialize};

use crate::json::serialize_field_as_string;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "BaseAccountJson", into = "BaseAccountJson")]
pub struct BaseAccount {
    pub address: String,
    pub pub_key: Option<PublicKey>,
    pub account_number: u64,
    pub sequence: u64,
}

impl BaseAccount {
    pub const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.BaseAccount";

    pub fn from_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str::<BaseAccount>(s)?)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("JSON serialization error")
    }
}

impl FromStr for BaseAccount {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_json(s)
    }
}

impl ToString for BaseAccount {
    fn to_string(&self) -> String {
        self.to_json()
    }
}

#[derive(Deserialize, Serialize)]
pub struct BaseAccountJson {
    #[serde(rename = "@type")]
    type_url: String,

    #[serde(with = "serialize_field_as_string")]
    pub account_number: u64,
    pub address: String,
    pub pub_key: Option<PublicKey>,
    #[serde(with = "serialize_field_as_string")]
    pub sequence: u64,
}

impl From<BaseAccount> for BaseAccountJson {
    fn from(base_account: BaseAccount) -> Self {
        BaseAccountJson::from(&base_account)
    }
}

impl From<&BaseAccount> for BaseAccountJson {
    fn from(base_account: &BaseAccount) -> Self {
        let type_url = BaseAccount::TYPE_URL.to_string();
        BaseAccountJson {
            type_url,
            address: base_account.address.to_owned(),
            pub_key: base_account.pub_key,
            account_number: base_account.account_number,
            sequence: base_account.sequence,
        }
    }
}

impl TryFrom<BaseAccountJson> for BaseAccount {
    type Error = Report;

    fn try_from(json: BaseAccountJson) -> Result<BaseAccount> {
        BaseAccount::try_from(&json)
    }
}

impl TryFrom<&BaseAccountJson> for BaseAccount {
    type Error = Report;

    fn try_from(json: &BaseAccountJson) -> Result<Self, Self::Error> {
        Ok(BaseAccount {
            account_number: json.account_number,
            address: json.address.to_owned(),
            pub_key: json.pub_key,
            sequence: json.sequence,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryAccountRequest {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryAccountResponse {
    pub account: Option<BaseAccount>,
}
