use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

const DENOM_UST: &str = "uusd";
const DENOM_LUNA: &str = "uluna";

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Token {
    #[serde(rename = "token")]
    Cw20 { contract_addr: Addr },
    #[serde(rename = "native_token")]
    Native { denom: String },
}

impl Token {
    pub fn is_native(&self) -> bool {
        matches!(self, Token::Native { .. })
    }

    pub fn luna() -> Self {
        Self::Native {
            denom: DENOM_LUNA.to_string(),
        }
    }

    pub fn is_luna(&self) -> bool {
        *self == Self::luna()
    }

    pub fn ust() -> Self {
        Self::Native {
            denom: DENOM_UST.to_string(),
        }
    }

    pub fn is_ust(&self) -> bool {
        *self == Self::ust()
    }
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Pair(pub [Token; 2]);

impl From<[Token; 2]> for Pair {
    fn from(input: [Token; 2]) -> Self {
        Self(input)
    }
}
