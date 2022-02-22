use cosmwasm_bignumber::Decimal256;
use tendermint::block;
use terrars::{dex, MsgExecuteContract};

use crate::accounts;

pub mod metrics;
pub mod multi_dex;

pub struct Trade {
    pub size: Decimal256,
    pub expected_profit: Decimal256,
    pub messages: Vec<MsgExecuteContract>,
}

#[async_trait::async_trait]
pub trait Strategy<L>: Send {
    /// Given the current prices of dex pairs a strategy should produce an ordered list of messages
    /// (most likely swaps) to be bundled into one atomic transaction.
    async fn execute(
        &self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools<L>,
    ) -> Vec<Trade>;
}
