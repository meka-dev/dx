use cosmwasm_std::Decimal;

use crate::dex;
use crate::strategy::Strategy;

pub struct Simple {}

impl Simple {
    pub fn new() -> Self {
        Self {}
    }
}

impl Strategy for Simple {
    fn execute(&self, pools: &dex::Pools, tokens: &dex::Tokens) -> Vec<terrars::Message> {
        // crazy arbs go here

        // Empty list for now.
        vec![]
    }
}
