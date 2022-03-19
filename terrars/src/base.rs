use std::str::FromStr as _;

use anyhow::{Error, Result};
use cosmrs::Denom;
use cosmwasm_bignumber::Uint256;
use serde::{Deserialize, Serialize};

use crate::proto;

pub mod abci;
pub mod query;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(try_from = "CoinJson", into = "CoinJson")]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Uint256,
}

impl Coin {
    pub const TYPE_URL: &'static str = "/cosmos.base.v1beta1.Coin";

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("JSON serialization error")
    }
}

impl ToString for Coin {
    fn to_string(&self) -> String {
        self.to_json()
    }
}

impl From<Coin> for cosmrs::Coin {
    fn from(input: Coin) -> Self {
        cosmrs::Coin::from(&input)
    }
}

impl From<&Coin> for cosmrs::Coin {
    fn from(input: &Coin) -> Self {
        Self {
            denom: input.denom.clone(),
            amount: cosmrs::Decimal::from(input.amount.0.as_u64()),
        }
    }
}

impl From<cosmrs::Coin> for Coin {
    fn from(input: cosmrs::Coin) -> Self {
        Coin::from(&input)
    }
}

impl From<&cosmrs::Coin> for Coin {
    fn from(input: &cosmrs::Coin) -> Self {
        Self {
            amount: input.amount.0.into(),
            denom: input.denom.clone(),
        }
    }
}

impl TryFrom<proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = cosmrs::ErrorReport;

    fn try_from(input: proto::cosmos::base::v1beta1::Coin) -> cosmrs::Result<Self> {
        Coin::try_from(&input)
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = cosmrs::ErrorReport;

    fn try_from(input: &proto::cosmos::base::v1beta1::Coin) -> cosmrs::Result<Self> {
        Ok(Self {
            amount: u128::from_str(&input.amount)?.into(),
            denom: input.denom.parse()?,
        })
    }
}

impl From<Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin::from(&coin)
    }
}

impl From<&Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: &Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin {
            denom: coin.denom.to_string(),
            amount: coin.amount.0.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct CoinJson {
    amount: String,
    denom: String,
}

impl From<Coin> for CoinJson {
    fn from(coin: Coin) -> Self {
        CoinJson::from(&coin)
    }
}

impl From<&Coin> for CoinJson {
    fn from(coin: &Coin) -> Self {
        // XXX: as_u64 can panic
        CoinJson {
            amount: coin.amount.0.to_string(),
            denom: coin.denom.to_string(),
        }
    }
}

impl TryFrom<CoinJson> for Coin {
    type Error = Error;

    fn try_from(json: CoinJson) -> Result<Coin> {
        Coin::try_from(&json)
    }
}

impl TryFrom<&CoinJson> for Coin {
    type Error = Error;

    fn try_from(json: &CoinJson) -> Result<Self, Self::Error> {
        Ok(Coin {
            amount: u128::from_str(&json.amount)?.into(),
            denom: Denom::from_str(&json.denom)
                .map_err(|err| anyhow::anyhow!("unable to parse Denom: {}", err))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use cosmrs::Denom;
    use cosmwasm_bignumber::Uint256;
    use prost::Message as _;
    use serde_json::json;

    use crate::proto;

    use super::Coin;

    #[test]
    fn coin_cosmrs_round_trip() {
        let want = Coin {
            amount: Uint256::from(2622376u128),
            denom: Denom::from_str("uusd").unwrap(),
        };

        let cosmrs_coin = cosmrs::Coin::from(&want);
        let have = Coin::from(cosmrs_coin);

        assert_eq!(have, want);
    }

    #[test]
    fn coin_json_round_trip() {
        assert_eq!(Coin::TYPE_URL, "/cosmos.base.v1beta1.Coin");

        let example_value = json!({
            "amount": "2622376",
            "denom": "uusd"
        });
        let example_json = example_value.to_string();
        let coin: Coin = serde_json::from_value(example_value).unwrap();
        assert_eq!(example_json, coin.to_string());
    }

    #[test]
    fn coin_json() {
        assert_eq!(Coin::TYPE_URL, "/cosmos.base.v1beta1.Coin");

        let example_value = json!({
            "amount": "2622376",
            "denom": "uusd"
        });
        let coin: Coin = serde_json::from_value(example_value).unwrap();
        assert_eq!(coin.amount, Uint256::from(2622376u128));
    }

    #[test]
    fn coin_protobuf_round_trip() {
        assert_eq!(Coin::TYPE_URL, "/cosmos.base.v1beta1.Coin");

        let want = Coin {
            amount: Uint256::from(2622376u128),
            denom: Denom::from_str("uusd").unwrap(),
        };
        let proto_msg = proto::cosmos::base::v1beta1::Coin::from(&want);
        let buf = proto_msg.encode_to_vec();
        let have =
            Coin::try_from(proto::cosmos::base::v1beta1::Coin::decode(&*buf).unwrap()).unwrap();

        assert_eq!(have, want);
    }
}

mod serialize_bytes_as_base64 {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: AsRef<[u8]>,
        S: Serializer,
    {
        serializer.collect_str(&base64::encode_config(value, base64::STANDARD))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        base64::decode(input.as_bytes()).map_err(de::Error::custom)
    }
}

mod serialize_field_as_string {
    use std::{fmt::Display, str::FromStr};

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}
