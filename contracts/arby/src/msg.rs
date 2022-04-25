use astroport::asset::AssetInfo;
use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// include postcondition

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub sprinters: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SetSprinters {
        sprinters: Vec<Addr>,
    },
    Sprint {
        input: Uint128,
        min_out: Uint128,
        path: Vec<(Addr, AssetInfo)>,
    },
    Withdraw {
        asset: AssetInfo,
        amount: Uint128,
        to: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
