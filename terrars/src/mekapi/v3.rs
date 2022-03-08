use anyhow::Result;
use cosmrs::{tendermint::block, Denom};
use cosmwasm_std::{Addr, Uint128};
use serde::{de::Error as _, Deserialize, Deserializer};

#[async_trait::async_trait]
pub trait Api: Send + Sync {
    async fn v3_pools(&self, factory: Addr, height: block::Height) -> Result<Vec<Pool>>;
}

#[derive(Debug, Deserialize)]
pub struct Pool {
    pub contract_address: Addr,
    pub liquidity_token: Addr,
    pub total_share: Uint128,
    pub assets: [Asset; 2],
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Asset {
    Cw20 {
        contract_addr: Addr,
        name: String,
        symbol: String,
        decimals: u64,
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
