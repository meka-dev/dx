use async_trait::async_trait;
use common::Result;
use cosmrs::{tendermint::block, AccountId, Denom};
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
    pub pools: Vec<Pool>,
}

fn denom_from_str<'de, D>(deserializer: D) -> Result<Denom, D::Error>
where
    D: Deserializer<'de>,
{
    let input: String = Deserialize::deserialize(deserializer)?;
    input.parse::<Denom>().map_err(D::Error::custom)
}
