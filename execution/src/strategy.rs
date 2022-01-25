use terrars::Any;

use crate::dex;

pub trait Strategy: Send {
    /// Given the current prices of dex pairs a strategy should produce an ordered list of messages (most likely swaps) to be bundled into one atomic transaction.
    fn execute(&self, pairs: &[dex::terraswap::Pair]) -> Vec<Any>;
}
