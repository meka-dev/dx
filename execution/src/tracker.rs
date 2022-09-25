use std::{
    collections::{HashMap, HashSet},
    ops::Deref as _,
};

use common::{Context as _, Result};
use sha2::{Digest as _, Sha256};
use strategy::{PoolId, Route};
use tendermint::{abci::transaction::Hash, block::Height, Block};

pub mod metrics;

const TIMEOUT_HEIGHT: u64 = 10;

pub trait Tracker {
    fn is_tracked(&self, route: &Route) -> Option<PoolId>;
    fn observe_block(&mut self, block: Block) -> Result<()>;
    fn track(&mut self, hash: Hash, route: &Route) -> Option<Route>;
}

/// Record to hold the route of the trade and the height when the tx entered the [`Tracker`]. The
/// latter is used to evict entries which have timed out, either by never making it into a block or
/// it being missed due to non-consecutive block events.
struct TxEntry {
    height: Height,
    route: Route,
}

/// In memory implementation of a [`Tracker`], which can be used for simple scenarios or test
/// cases where persistence across binary restarts is not required.
///
/// NB(xla): It would be favoruable if we use the BTree variants from [`std::collections`], but
/// unfortunately [`tendermint::abci::transaction::Hash`] does not implement `Ord`.
#[derive(Default)]
pub struct InMemory {
    addr_index: HashMap<PoolId, HashSet<Hash>>,
    latest_height: Height,
    tx_index: HashMap<Hash, TxEntry>,
}

impl Tracker for InMemory {
    fn is_tracked(&self, route: &Route) -> Option<PoolId> {
        for addr in route.iter() {
            if self.addr_index.contains_key(addr) {
                return Some(addr.clone());
            }
        }

        None
    }

    fn observe_block(&mut self, block: Block) -> Result<()> {
        for tx in block.data.iter() {
            let hash = Hash::new(
                <[u8; 32]>::try_from(Sha256::digest(tx).deref())
                    .context("failed to hash transaction")?,
            );
            self.untrack(hash);
        }

        self.latest_height = block.header.height;
        self.addr_index.retain(|_, txs| !txs.is_empty());

        // Evict all transactions which have been around for longer than the configured timeout.
        self.tx_index
            .retain(|_, entry| self.latest_height.value() - entry.height.value() < TIMEOUT_HEIGHT);

        Ok(())
    }

    fn track(&mut self, hash: Hash, route: &Route) -> Option<Route> {
        for addr in route {
            let txs = self
                .addr_index
                .entry(addr.clone())
                .or_insert_with(HashSet::new);
            txs.insert(hash);
        }

        self.tx_index
            .insert(
                hash,
                TxEntry {
                    height: self.latest_height,
                    route: route.to_vec(),
                },
            )
            .map(|e| e.route)
    }
}

impl InMemory {
    fn untrack(&mut self, hash: Hash) {
        if let Some(entry) = self.tx_index.remove(&hash) {
            for addr in &entry.route {
                if let Some(idx) = self.addr_index.get_mut(addr) {
                    idx.remove(&hash);
                }
            }
        }
    }
}
