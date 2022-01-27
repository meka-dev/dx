use cosmwasm_std::Decimal;

use crate::dex;
use crate::strategy::Strategy;

#[derive(Default)]
pub struct SinglePair {}

impl Strategy for SinglePair {
    fn execute(&self, pools: &dex::Pools, tokens: &dex::Tokens) -> Vec<terrars::Message> {
        // crazy arbs go here

        // Empty list for now.
        vec![]
    }
}
