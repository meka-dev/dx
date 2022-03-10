use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;
use tendermint::block::{self};
use terrars::{dex::Pools, MsgExecuteContract};

use crate::accounts;

pub mod fetcher;
pub mod metrics;
pub mod multi_dex;

pub type Route = Vec<Addr>;

#[derive(Debug)]
pub struct Trade {
    pub size: Decimal256,
    pub expected_profit: Decimal256,
    pub messages: Vec<MsgExecuteContract>,
    pub route: Route,
}

#[async_trait::async_trait]
pub trait Strategy: Send {
    /// Given the current prices of dex pairs a strategy should produce an ordered list of messages
    /// (most likely swaps) to be bundled into one atomic transaction.
    async fn execute(
        &mut self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &Pools,
    ) -> Vec<Trade>;
}
