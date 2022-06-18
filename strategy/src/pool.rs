use std::collections::HashMap;

use cosmwasm_bignumber::Decimal256;

use crate::{asset::WeightedAsset, Token};

pub type PoolId = String;

#[derive(Clone, Debug)]
pub struct Pool {
    pub id: PoolId,
    pub assets: [WeightedAsset; 2], // ;2 needs to be generalized
    pub swap_fee_bid: Decimal256,
    pub swap_fee_ask: Decimal256,
}

impl PartialEq for Pool {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Pool {}

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

        // common::tracing::debug!("bid_reserve {:}, ask_reserve {:}", bid_reserve, ask_reserve);
        // common::tracing::debug!("(self.swap_fee_bid * self.swap_fee_ask * ask_reserve * amount) /
        // bid_reserve {:}", (self.swap_fee_bid * self.swap_fee_ask * ask_reserve * amount) /
        // bid_reserve); common::tracing::debug!("(amount * self.swap_fee_bid) {:}", (amount
        // * self.swap_fee_bid));
        Some(
            (self.swap_fee_bid * self.swap_fee_ask * ask_reserve * amount)
                / (bid_reserve + amount * self.swap_fee_bid),
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
            _ => None,
        }
    }
}

// pub type Pools<A, P> = Vec<Pool<A, P>>;
pub type Pools = Vec<Pool>;

// type PoolMap<Id> = HashMap<Id, Pool>;
pub type PoolMap = HashMap<PoolId, Pool>;
