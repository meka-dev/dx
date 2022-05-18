use std::{fmt, str::FromStr};

use common::{Report, Result};
use cosmrs::Decimal;
use serde::{Deserialize, Serialize};

use crate::proto;

/// Coin defines a token with a denomination and an amount.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Decimal,
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

pub mod abci {
    use crate::{proto::cosmos::base::abci, MsgProto};

    impl MsgProto for abci::v1beta1::MsgData {
        const TYPE_URL: &'static str = "/cosmos.base.abci.v1beta1.MsgData";
    }
}

pub mod reflection {
    use crate::{proto::cosmos::base::reflection, MsgProto};

    impl MsgProto for reflection::v2alpha1::MsgDescriptor {
        const TYPE_URL: &'static str = "/cosmos.base.reflection.v2alpha1.MsgDescriptor";
    }
}
