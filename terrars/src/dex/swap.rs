use cosmwasm_std::{Addr, Decimal, Uint128};
pub use cw20::Cw20ExecuteMsg;
use serde::Deserialize;
pub use terraswap::{
    asset::AssetInfo,
    pair::{Cw20HookMsg, ExecuteMsg},
    router::ExecuteMsg as RouterExecuteMsg,
};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Target {
    Cw20ExecuteMsg(Cw20ExecuteMsg),
    ExecuteMsg(ExecuteMsg),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SendTarget {
    Cw20HookMsg(Cw20HookMsg),
    RouterExecuteMsg(RouterExecuteMsg),
}

#[derive(Deserialize, Debug)]
pub struct Swap {
    pub contract_address: Addr,
    pub offer_asset: AssetInfo,
    pub amount: Uint128,
    pub belief_price: Option<Decimal>,
    pub max_spread: Option<Decimal>,
}

// struct RouterSwap {}

#[derive(Deserialize, Debug)]
pub enum Action {
    Swap(Swap),
}
