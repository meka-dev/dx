use async_trait::async_trait;
use common::Result;
use cosmrs::{tendermint::block, Denom};
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Addr, Uint128};
use serde::{de::Error as _, Deserialize, Deserializer};
use terrars::dex;

#[async_trait]
pub trait Api: Send + Sync {
    async fn v3_pools(&self, factory: Addr, height: block::Height) -> Result<Vec<Pool>>;
}

#[derive(Clone, Debug, Deserialize)]
pub struct Pool {
    pub contract_address: Addr,
    pub liquidity_token: Addr,
    pub total_share: Uint128,
    pub assets: [Asset; 2],
}

impl From<Pool> for dex::Pool {
    fn from(input: Pool) -> Self {
        Self {
            assets: [
                dex::Asset::from(input.assets[0].clone()),
                dex::Asset::from(input.assets[1].clone()),
            ],
            contract_address: input.contract_address,
            liquidity_token: input.liquidity_token,
            total_share: input.total_share,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Asset {
    Cw20 {
        contract_addr: Addr,
        name: String,
        symbol: String,
        decimals: u8,
        amount: Uint128,
        total_supply: Uint128,
    },
    Native {
        #[serde(deserialize_with = "denom_from_str")]
        denom: Denom,
        amount: Uint128,
        total_supply: Uint128,
    },
}

impl From<&Pool> for strategy::Pool {
    fn from(p: &Pool) -> Self {
        strategy::Pool {
            id: p.contract_address.to_string(),
            assets: p.assets.clone().map(strategy::WeightedAsset::from),
            swap_fee_bid: Decimal256::permille(1000),
            swap_fee_ask: Decimal256::permille(997),
        }
    }
}

impl From<Pool> for strategy::Pool {
    fn from(p: Pool) -> Self {
        From::from(&p)
    }
}

impl From<Asset> for strategy::WeightedAsset {
    fn from(a: Asset) -> Self {
        match a {
            Asset::Cw20 {
                contract_addr,
                decimals,
                amount,
                ..
            } => strategy::WeightedAsset {
                token: strategy::Token {
                    id: contract_addr.to_string(),
                    decimals,
                    native: false,
                },
                amount: Decimal256::from_uint256(amount.u128())
                    / Decimal256::from_uint256(10u128.pow(decimals as u32)),
                weight: 1u128.into(),
            },
            Asset::Native { denom, amount, .. } => strategy::WeightedAsset {
                token: strategy::Token {
                    id: denom.to_string(),
                    decimals: 6,
                    native: true,
                },
                amount: Decimal256::from_uint256(amount.u128())
                    / Decimal256::from_uint256(10u128.pow(6)),
                weight: 1u128.into(),
            },
        }
    }
}

impl From<Asset> for dex::Asset {
    fn from(input: Asset) -> Self {
        match input {
            Asset::Cw20 {
                contract_addr,
                decimals,
                amount,
                ..
            } => Self {
                amount,
                amount_decimal: dex::Asset::to_scaled_decimal256(amount.u128(), decimals as u32),
                decimals,
                token: dex::Token::Cw20 { contract_addr },
            },
            Asset::Native { denom, amount, .. } => Self {
                amount,
                amount_decimal: dex::Asset::to_scaled_decimal256(amount.u128(), 6),
                decimals: 6,
                token: dex::Token::Native {
                    denom: denom.to_string(),
                },
            },
        }
    }
}

#[derive(Deserialize)]
pub struct PoolsResponse {
    pub height: block::Height,
    pub dex: String,
    pub factory: Addr,
    pub pools: Vec<Pool>,
}

// NB(xla): There are cases where the denom is bogus, e.g. `1000000` and then there is the lack of
// support for ibc assets in the parsing of `Denom`, both cases are not relevant for us at this
// point.
fn denom_from_str<'de, D>(deserializer: D) -> Result<Denom, D::Error>
where
    D: Deserializer<'de>,
{
    let fallback = "undefined".parse::<Denom>().map_err(D::Error::custom)?;
    let input: String = Deserialize::deserialize(deserializer)?;
    Ok(input.parse::<Denom>().unwrap_or(fallback))
}
