use cosmwasm_std::Uint128;
use tendermint::block;
use terrars::MsgExecuteContract;

use crate::{accounts, dex};

pub mod metrics;
pub mod multi_dex;

pub type Trade = (Uint128, Uint128, Vec<MsgExecuteContract>);

#[async_trait::async_trait]
pub trait Strategy: Send {
    /// Given the current prices of dex pairs a strategy should produce an ordered list of messages
    /// (most likely swaps) to be bundled into one atomic transaction.
    async fn execute(
        &self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools,
    ) -> Vec<Trade>;
}
