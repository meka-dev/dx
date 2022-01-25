use cosmwasm_std::Uint128;

use terrars::{Lcd, Message};

pub mod astroport;
pub mod terraswap;

#[derive(Clone, Debug, PartialEq)]
pub enum AssetInfo {
    NativeToken { denom: String },
    Token { contract_addr: String },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Asset {
    pub amount: Uint128,
    pub info: AssetInfo,
}

#[async_trait::async_trait]
pub trait Pool: Send + Sync {
    fn coins(&self) -> [Asset; 2];
    fn swap(&self) -> Message;
    async fn update(&mut self, lcd: &Lcd) -> anyhow::Result<()>;
}
