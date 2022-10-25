use std::{fmt, hash::Hash, str::FromStr};

use common::{err, Report, Result, WrapErr};
use cosmrs::{tendermint, Decimal};
use serde::{de, de::Error as _, ser, Deserialize, Serialize};
use subtle_encoding::bech32;

use crate::proto;

pub mod abci;
pub mod query;

/// Maximum allowed length (in bytes) for an address.
pub const MAX_ADDRESS_LENGTH: usize = 255;

/// Account identifiers
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct AccountId {
    /// Account ID encoded as Bech32
    bech32: String,

    /// Length of the human-readable prefix of the address
    hrp_length: usize,
}

impl AccountId {
    /// Create an [`AccountId`] with the given human-readable prefix and
    /// public key hash.
    pub fn new(prefix: &str, bytes: &[u8]) -> Result<Self> {
        let id = bech32::encode(prefix, bytes);

        // NB(tarcieri): ensure this is the proper validation for an account prefix
        if !prefix.chars().all(|c| matches!(c, 'a'..='z')) {
            return Err(err!(
                "expected prefix to be lowercase alphabetical characters only: {:}",
                id
            ));
        }

        if matches!(bytes.len(), 1..=MAX_ADDRESS_LENGTH) {
            Ok(Self {
                bech32: id,
                hrp_length: prefix.len(),
            })
        } else {
            Err(err!(
                "account ID should be at most {} bytes long, but was {} bytes long",
                MAX_ADDRESS_LENGTH,
                bytes.len()
            ))
        }
    }

    /// Get the human-readable prefix of this account.
    pub fn prefix(&self) -> &str {
        &self.bech32[..self.hrp_length]
    }

    /// Decode an account ID from Bech32 to an inner byte value.
    pub fn to_bytes(&self) -> Vec<u8> {
        bech32::decode(&self.bech32)
            .expect("malformed Bech32 AccountId")
            .1
    }
}

impl AsRef<str> for AccountId {
    fn as_ref(&self) -> &str {
        &self.bech32
    }
}

impl fmt::Debug for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AccountId").field(&self.as_ref()).finish()
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for AccountId {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (hrp, bytes) = bech32::decode(s).wrap_err(format!("invalid bech32: '{s}'"))?;
        Self::new(&hrp, &bytes)
    }
}

impl TryFrom<AccountId> for tendermint::account::Id {
    type Error = Report;

    fn try_from(id: AccountId) -> Result<tendermint::account::Id> {
        tendermint::account::Id::try_from(&id)
    }
}

// NB(tarcieri): non-fixed-width account ID type
impl TryFrom<&AccountId> for tendermint::account::Id {
    type Error = Report;

    fn try_from(id: &AccountId) -> Result<tendermint::account::Id> {
        let bytes = id.to_bytes();
        let len = bytes.len();

        match bytes.try_into() {
            Ok(bytes) => Ok(tendermint::account::Id::new(bytes)),
            _ => Err(err!(
                "invalid length {} for account ID: {:}",
                len,
                id.bech32
            )),
        }
    }
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl Serialize for AccountId {
    fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bech32.serialize(serializer)
    }
}

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(try_from = "CoinJson", into = "CoinJson")]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Decimal, // NB(pm): this type is only u64 and the cosmos sdk can do 255 bit
}

impl TryFrom<proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = Report;

    fn try_from(proto: proto::cosmos::base::v1beta1::Coin) -> Result<Coin> {
        Coin::try_from(&proto)
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = Report;

    fn try_from(proto: &proto::cosmos::base::v1beta1::Coin) -> Result<Coin> {
        Ok(Coin {
            denom: proto.denom.parse()?,
            amount: proto.amount.parse()?,
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
            amount: coin.amount.to_string(),
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
    type Error = Report;

    fn try_from(json: CoinJson) -> Result<Coin> {
        Coin::try_from(&json)
    }
}

impl TryFrom<&CoinJson> for Coin {
    type Error = Report;

    fn try_from(json: &CoinJson) -> Result<Self, Self::Error> {
        Ok(Coin {
            amount: Decimal::from_str(&json.amount)
                .map_err(|err| err!("unable to parse amount: {}", err))?,
            denom: Denom::from_str(&json.denom)
                .map_err(|err| err!("unable to parse Denom: {}", err))?,
        })
    }
}

/// Denomination.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Denom(String);

impl AsRef<str> for Denom {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Denom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl FromStr for Denom {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars()
            .all(|c| matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '/'))
        {
            Ok(Denom(s.to_owned()))
        } else {
            Err(cosmrs::Error::Denom { name: s.to_owned() }.into())
        }
    }
}

pub mod reflection {
    use crate::{proto::cosmos::base::reflection, MsgProto};

    impl MsgProto for reflection::v2alpha1::MsgDescriptor {
        const TYPE_URL: &'static str = "/cosmos.base.reflection.v2alpha1.MsgDescriptor";
    }
}
