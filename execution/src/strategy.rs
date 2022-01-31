use terrars::Message;

use crate::dex;

pub mod multi_dex;

pub trait Strategy: Send {
    /// Given the current prices of dex pairs a strategy should produce an ordered list of messages
    /// (most likely swaps) to be bundled into one atomic transaction.
    fn execute(&self, pools: &dex::Pools) -> Vec<Message>;
}
