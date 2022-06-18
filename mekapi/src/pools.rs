use std::str::FromStr as _;

use async_trait::async_trait;
use common::{err, tracing::error, Report, Result};
use cosmrs::{tendermint::block, AccountId, Denom};
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::Uint128;
use osmors::ASSET_DECIMALS;
use serde::{
    de::{Deserializer, Error as _},
    Deserialize,
    Serialize,
};

#[async_trait]
pub trait Api: Send + Sync {
    async fn v5(&self, height: block::Height) -> Result<Vec<PoolV5>>;
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
pub struct V5Response {
    pub height: block::Height,
    pub pools: Vec<PoolV5>,
}

impl TryFrom<&PoolV5> for strategy::Pool {
    type Error = Report;

    fn try_from(p: &PoolV5) -> Result<Self> {
        let ass: Vec<_> = p.assets.iter().try_fold(
            vec![],
            |mut a, e| -> Result<Vec<strategy::WeightedAsset>> {
                let aw = asset_to_weighted(e)?;
                a.push(aw);
                Ok(a)
            },
        )?;
        if ass.len() != 2 {
            error!("pool {:?}", p);
            return Err(err!("can only handle pools with two assets right now"));
        }
        Ok(strategy::Pool {
            id: p.id.clone(),
            assets: ass.try_into().unwrap(), // adjust?
            swap_fee_bid: Decimal256::one() - Decimal256::from_str(&p.swap_fee)?,
            swap_fee_ask: Decimal256::permille(1000),
        })
    }
}

fn asset_to_weighted(a: &WeightedAsset) -> Result<strategy::WeightedAsset> {
    let d = ASSET_DECIMALS
        .get(&a.token.denom.to_string())
        .ok_or_else(|| err!("could not find decimals for {}", a.token.denom))?;
    Ok(strategy::WeightedAsset {
        token: strategy::Token {
            id: a.token.denom.to_string(),
            decimals: *d,
            native: false,
        },
        amount: Decimal256::from_ratio(
            Uint256::from(a.token.amount.u128()),
            Uint256::from(10u128.pow(*d as u32)),
        ), // XXX
        weight: a.weight.u128(),
    })
}

impl TryFrom<PoolV5> for strategy::Pool {
    type Error = Report;

    fn try_from(p: PoolV5) -> Result<Self> {
        TryFrom::try_from(&p)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PoolV5 {
    pub address: AccountId,
    pub id: String,
    pub assets: Vec<WeightedAsset>,
    // #[serde(deserialize_with = "decimal_from_str")]
    // pub swap_fee: Decimal,
    // #[serde(deserialize_with = "decimal_from_str")]
    // pub exit_fee: Decimal,
    pub swap_fee: String,
    pub exit_fee: String,
    pub total_shares: Uint256,
    pub total_weight: Uint128,
}

fn denom_from_str<'de, D>(deserializer: D) -> Result<Denom, D::Error>
where
    D: Deserializer<'de>,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    input.parse::<Denom>().map_err(D::Error::custom)
}
