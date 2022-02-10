use tendermint::block;
use terrars::MsgExecuteContract;

use crate::{accounts, dex};

pub mod metrics;
pub mod multi_dex;

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
    ) -> Vec<Vec<MsgExecuteContract>>;
}
