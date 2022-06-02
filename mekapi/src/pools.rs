use std::str::FromStr;

use async_trait::async_trait;
use chainrs::cosmos::base::{AccountId, Denom};
use common::Result;
use cosmrs::tendermint::block;
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Uint128;
use serde::{
    de::{Deserializer, Error as _},
    Deserialize,
    Serialize,
};

#[async_trait]
pub trait Api: Send + Sync {
    async fn v4(&self, height: block::Height) -> Result<Vec<Pool>>;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Pool {
    pub address: AccountId,
    pub id: String,
    pub assets: Vec<WeightedAsset>,
    // #[serde(deserialize_with = "decimal_from_str")]
    // pub swap_fee: Decimal,
    // #[serde(deserialize_with = "decimal_from_str")]
    // pub exit_fee: Decimal,
    pub swap_fee: String,
    pub exit_fee: String,
    pub total_shares: Asset,
    pub total_weight: Uint128,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Asset {
    #[serde(deserialize_with = "denom_from_str")]
    pub denom: Denom,
    pub amount: Uint128,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct WeightedAsset {
    pub token: Asset,
    pub weight: Uint128,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct V4Response {
    pub height: block::Height,
    pub txhash: String,
    pub pools: Vec<Pool>,
}

impl From<&Pool> for strategy::Pool {
    fn from(p: &Pool) -> Self {
        strategy::Pool {
            id: p.id.clone(),
            assets: [
                p.assets[0].clone().into(),
                p.assets[1].clone().into(),
            ],
            swap_fee_bid: Decimal256::one() - Decimal256::from_str(&p.swap_fee).unwrap(),
            swap_fee_ask: Decimal256::permille(1000),
        }
    }
}

impl From<Pool> for strategy::Pool {
    fn from(p: Pool) -> Self {
        From::from(&p)
    }
}

impl From<WeightedAsset> for strategy::WeightedAsset {
    fn from(a: WeightedAsset) -> Self {
        let decimals = 18u8; // XXX: ?
        strategy::WeightedAsset {
                token: strategy::Token {
                    id: a.token.denom.to_string(),
                    decimals,
                    native: false,
                },
                amount: Decimal256::from_uint256(a.token.amount.u128())
                    / Decimal256::from_uint256(10u128.pow(decimals as u32)),
                weight: a.weight.into(),
            }
    }
}

fn denom_from_str<'de, D>(deserializer: D) -> Result<Denom, D::Error>
where
    D: Deserializer<'de>,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    input.parse::<Denom>().map_err(D::Error::custom)
}
