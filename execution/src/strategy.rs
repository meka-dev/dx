use cosmwasm_bignumber::Uint256;
use cosmwasm_std::Addr;
use tendermint::block::{self};
use terrars::{dex::Pools, wasm::MsgExecuteContract};

use crate::accounts;

pub mod fetcher;
pub mod metrics;
pub mod multi_dex;

pub type Route = Vec<Addr>;

#[derive(Clone, Debug)]
pub struct Trade {
    pub size: Uint256,
    pub expected_profit: Uint256,
    pub messages: Vec<MsgExecuteContract>,
    pub route: Route,
}

#[async_trait::async_trait]
pub trait Strategy: Send {
    async fn execute_mempool_tx(
        &mut self,
        height: block::Height,
        sender: &terrars::AccountId,
        pools: &[mekapi::v3::Pool],
    ) -> Vec<Trade>;

    /// Given the current prices of dex pairs a strategy should produce an ordered list of messages
    /// (most likely swaps) to be bundled into one atomic transaction.
    async fn execute_pools(
        &mut self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &Pools,
    ) -> Vec<Trade>;
}
