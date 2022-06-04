use cosmwasm_bignumber::{Decimal256, Uint256};

use crate::token::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeightedAsset {
    pub token: Token,
    pub amount: Decimal256,
    pub weight: Uint256,
}
