use cosmwasm_bignumber::Decimal256;
use tendermint::block;

use crate::{Pool, PoolId, Pools, Token};

pub type Route = Vec<PoolId>;

#[derive(Clone, Debug)]
pub struct Leg {
    pub in_amount: Decimal256,
    pub in_token: Token,
    pub min_out_amount: Option<Decimal256>,
    pub pool: Pool,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub size: Decimal256,
    pub expected_profit: Decimal256,
    pub legs: Vec<Leg>,
    pub route: Route,
}

#[async_trait::async_trait]
pub trait Strategy: Send {
    async fn execute_mempool_tx(&mut self, height: block::Height, pools: &Pools) -> Vec<Trade>;

    /// Given the current prices of ex pairs a strategy should produce an ordered list of messages
    /// (most likely swaps) to be bundled into one atomic transaction.
    async fn execute_pools(
        &mut self,
        height: block::Height,
        funds: Decimal256,
        pools: &Pools,
    ) -> Vec<Trade>;
}
