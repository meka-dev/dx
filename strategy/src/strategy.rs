use std::collections::HashMap;

use cosmwasm_bignumber::{Decimal256, Uint256};
use tendermint::block;

use crate::{Pool, PoolId, Pools, Token};

pub type Balances = HashMap<Token, Uint256>;

pub type Route = Vec<PoolId>;

#[derive(Clone, Debug)]
pub struct Leg {
    pub in_amount: Decimal256,
    pub out_token: Token,
    pub pool: Pool,
}

#[derive(Clone, Debug)]
pub struct Trade {
    pub size: Decimal256,
    pub expected_profit: Decimal256,
    pub in_token: Token,
    pub legs: Vec<Leg>,
    pub route: Route,
}

impl Trade {
    pub fn size_to_int(&self) -> Uint256 {
        Uint256(self.size.0 / Uint256::from(10u128.pow(18 - self.in_token.decimals as u32)).0)
    }

    pub fn expected_profit_to_int(&self) -> Uint256 {
        // XXX: assumes in token == out token
        Uint256(
            self.expected_profit.0
                / Uint256::from(10u128.pow(18 - self.in_token.decimals as u32)).0,
        )
    }
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
