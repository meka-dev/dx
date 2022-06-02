use std::{collections::HashMap, hash::Hash};

use chainrs::tendermint::block::{self};
use cosmwasm_bignumber::{Decimal256, Uint256};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Token {
    pub id: String, // Denom?
    pub decimals: u8,
    pub native: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeightedAsset {
    pub token: Token,
    pub amount: Decimal256,
    pub weight: Uint256,
}

pub type PoolId = String;

#[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Pool<A, P>
// where P: Hash + Eq + Ord + PartialEq,
// A: Hash + Eq + Ord + PartialEq
pub struct Pool {
    pub id: PoolId,
    pub assets: [WeightedAsset; 2], // ;2 needs to be generalized
    pub swap_fee_bid: Decimal256,
    pub swap_fee_ask: Decimal256,
}

impl Pool {
    // Assumes XYK
    pub fn swap(&self, tok: &Token, amount: Decimal256) -> Option<Decimal256> {
        if amount.is_zero() {
            return None;
        }

        let (bid_reserve, ask_reserve) = match &self.assets {
            [bid, ask] if bid.token == *tok => (bid.amount, ask.amount),
            [ask, bid] if bid.token == *tok => (bid.amount, ask.amount),
            _ => return None,
        };

        Some(
            ask_reserve
                - (self.swap_fee_bid * self.swap_fee_ask * ask_reserve * amount) / bid_reserve
                + (amount * self.swap_fee_bid),
        )
    }

    pub fn has_asset_id(&self, id: String) -> bool {
        self.assets[0].token.id == id || self.assets[1].token.id == id
    }

    pub fn has_asset(&self, t: &Token) -> bool {
        self.assets[0].token == *t || self.assets[1].token == *t
    }

    pub fn ask_asset(&self, t: &Token) -> Option<Token> {
        match &self.assets {
            [bid, ask] if bid.token == *t => Some(ask.token.clone()),
            [ask, bid] if bid.token == *t => Some(ask.token.clone()),
            _ => return None,
        }
    }
}

// pub type Pools<A, P> = Vec<Pool<A, P>>;
pub type Pools = Vec<Pool>;

// type PoolMap<Id> = HashMap<Id, Pool>;
pub type PoolMap = HashMap<PoolId, Pool>;

pub type Route = Vec<PoolId>;

#[derive(Clone, Debug)]
// pub struct Leg<A, P>
// where P: Hash + Eq + Ord + PartialEq,
// A: Hash + Eq + Ord + PartialEq
pub struct Leg {
    pub in_amount: Decimal256,
    pub in_token: Token,
    pub min_out_amount: Option<Decimal256>,
    pub pool: Pool,
}

#[derive(Clone, Debug)]
// pub struct Trade<A, P>
// where P: Hash + Eq + Ord + PartialEq,
// A: Hash + Eq + Ord + PartialEq
pub struct Trade {
    pub size: Decimal256,
    pub expected_profit: Decimal256,
    pub legs: Vec<Leg>,
    pub route: Route,
}

#[async_trait::async_trait]
// pub trait Strategy<A, P>: Send
// where P: Hash + Eq + Ord + PartialEq,
// A: Hash + Eq + Ord + PartialEq
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
