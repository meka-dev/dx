use std::hash::{Hash, Hasher};

use astroport::asset::AssetInfo;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Addr, Uint128};
use serde::Deserialize;
use tracing::trace;

use crate::dex::{Asset, Token, TokenPair};

#[derive(Clone, Debug, Deserialize, Eq)]
pub struct Pool {
    pub assets: [Asset; 2],
    pub contract_address: Addr,
    pub liquidity_token: Addr,
    pub total_share: Uint128,
}

impl Hash for Pool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contract_address.hash(state);
    }
}

impl PartialEq for Pool {
    fn eq(&self, other: &Self) -> bool {
        self.contract_address == other.contract_address
    }
}

impl Pool {
    pub fn ask_token(&self, bid_token: &Token) -> Token {
        if self.assets[0].token == *bid_token {
            self.assets[1].token.clone()
        } else {
            self.assets[0].token.clone()
        }
    }

    pub fn has_token(&self, tok: &Token) -> bool {
        self.assets[0].token == *tok || self.assets[1].token == *tok
    }

    pub fn compute_swap(
        &self,
        bid_token: &Token,
        ask_token: &Token,
        amount: Uint256,
    ) -> Option<(Uint256, Uint256)> {
        let (bid_asset, ask_asset) = match &self.assets {
            [bid, ask] if bid.token == *bid_token && ask.token == *ask_token => (bid, ask),
            [ask, bid] if bid.token == *bid_token && ask.token == *ask_token => (bid, ask),
            _ => return None,
        };

        let ask_reserve: Uint256 = ask_asset.amount.into();
        let bid_reserve: Uint256 = bid_asset.amount.into();

        if amount.is_zero() || bid_reserve.is_zero() || ask_reserve.is_zero() {
            return None;
        }

        let cp = ask_reserve * bid_reserve;
        trace!("cp {:?}", cp);
        let ret: Uint256 = (Decimal256::from_uint256(ask_reserve)
            - Decimal256::from_ratio(cp, bid_reserve + amount))
            * Uint256::one();
        trace!("ret {:?}", ret);
        let spread: Uint256 = (amount * Decimal256::from_ratio(ask_reserve, bid_reserve)) - ret;
        trace!("spread {:?}", spread);
        let fee = ret * Decimal256::permille(3); // XXX: get from chain
        trace!("fee {:?}", fee);
        let out = ret - fee;
        trace!("out {:?}", out);

        Some((out, spread))
    }

    pub fn swap_arby(&self, token: &Token) -> (Addr, AssetInfo) {
        (self.contract_address.clone(), (*token).clone().into())
    }

    pub fn token_pair(&self) -> TokenPair {
        [self.assets[0].token.clone(), self.assets[1].token.clone()].into()
    }
}

pub struct Pools(pub Vec<Pool>);

impl From<Vec<Pool>> for Pools {
    fn from(input: Vec<Pool>) -> Self {
        Self(input)
    }
}
