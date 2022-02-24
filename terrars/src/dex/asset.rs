use anyhow::Error;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::Uint128;
use serde::Deserialize;

use crate::dex::Token;

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "AssetJson")]
pub struct Asset {
    pub token: Token,
    pub amount: Uint128,
    pub amount_decimal: Decimal256,
    pub decimals: u8,
}

impl Asset {
    pub fn min_val(&self) -> Decimal256 {
        Decimal256::from_ratio(
            Uint256::one(),
            Into::<Uint256>::into(10u128.pow(self.decimals as u32)),
        )
    }
}

impl Eq for Asset {}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl Ord for Asset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.token.cmp(&other.token)
    }
}

impl PartialOrd for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(Token, Uint128, u8)> for Asset {
    fn from((token, amount, decimals): (Token, Uint128, u8)) -> Self {
        let amount_decimal = to_scaled_decimal256(amount.u128(), decimals as u32);

        Self {
            token,
            amount,
            amount_decimal,
            decimals,
        }
    }
}

fn to_scaled_decimal256<T: Into<Uint256>>(n: T, decimals: u32) -> Decimal256 {
    Decimal256::from_uint256(n) / Decimal256::from_uint256(10u128.pow(decimals))
}

#[derive(Clone, Debug, Deserialize)]
struct AssetJson {
    amount: Uint128,
    info: Token,
}

impl TryFrom<AssetJson> for Asset {
    type Error = Error;

    fn try_from(input: AssetJson) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

impl TryFrom<&AssetJson> for Asset {
    type Error = Error;

    fn try_from(input: &AssetJson) -> Result<Self, Self::Error> {
        // FIXME(xla): Mekapi currently doesn't consistently give use the decimals.
        let decimals = 6;
        Ok(Self {
            amount: input.amount,
            amount_decimal: to_scaled_decimal256(input.amount.u128(), decimals as u32),
            token: input.info.clone(),
            decimals,
        })
    }
}
