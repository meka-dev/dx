use astroport::asset::AssetInfo as AstroAssetInfo;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use terraswap::asset::AssetInfo;

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

impl From<AssetInfo> for Token {
    fn from(input: AssetInfo) -> Self {
        match input {
            AssetInfo::Token { contract_addr } => Token::Cw20 {
                contract_addr: Addr::unchecked(contract_addr),
            },
            AssetInfo::NativeToken { denom } => Token::Native { denom },
        }
    }
}

impl From<Token> for AstroAssetInfo {
    fn from(input: Token) -> Self {
        match input {
            Token::Cw20 { contract_addr } => AstroAssetInfo::Token {
                contract_addr: Addr::unchecked(contract_addr),
            },
            Token::Native { denom } => AstroAssetInfo::NativeToken { denom },
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Pair(pub [Token; 2]);

impl Pair {
    pub fn other(&self, t: &Token) -> Option<&Token> {
        match &self.0 {
            [target, other] if t == target => Some(other),
            [other, target] if t == target => Some(other),
            _ => None,
        }
    }
}

impl From<[Token; 2]> for Pair {
    fn from(input: [Token; 2]) -> Self {
        Self(input)
    }
}
