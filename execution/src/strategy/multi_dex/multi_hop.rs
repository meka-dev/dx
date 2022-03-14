#![allow(clippy::borrowed_box)]

use std::collections::{HashMap, HashSet, VecDeque};

use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;
use indexmap::IndexSet;
use itertools::Itertools;
use tendermint::block;
use xxhash_rust::xxh3::Xxh3Builder;

use common::tracing::{debug, error};
use terrars::dex::{self, Pool, Token};

use crate::{
    accounts,
    strategy::{
        metrics,
        multi_dex::utils::{compute_cycle_liq, trade_path},
        Strategy, Trade,
    },
};

// type XxIndexMap<K, V> = IndexMap<K, V, Xxh3Builder>;
type XxIndexSet<K> = IndexSet<K, Xxh3Builder>;

// Adjacency map for token -> pools
type TokenAdjacencyMap = HashMap<Token, HashSet<Addr>>;

// Mapping to allow efficient pool data look-up
type PoolMap<'a> = HashMap<Addr, &'a Pool>;

#[derive(Default)]
pub struct MultiHop {
    num_pools: usize,
    paths: Vec<Vec<Addr>>,
}

impl metrics::Labeled for MultiHop {
    const MODULE: metrics::Module = metrics::Module::MultiDex;
    const NAME: metrics::Name = metrics::Name::MultiHop;
}

const MAX_PATH_LEN: usize = 5;

impl MultiHop {
    fn dfs(
        &self,
        start: &Token,
        pool_map: &PoolMap<'_>,
        token_adjacencies: &TokenAdjacencyMap,
    ) -> Vec<Vec<Addr>> {
        let mut pool_paths: Vec<Vec<Addr>> = vec![];
        let mut pool_trail: XxIndexSet<&Addr> = XxIndexSet::with_hasher(Xxh3Builder::new());
        // a queue for every level/node in the tree, e.g. a queue of queues
        let mut pool_q: VecDeque<VecDeque<&Addr>> = VecDeque::new();

        let mut src = start.clone();
        let mut dst: Token;

        // token_adjacencies.get(src) returns all pools containing start tokens
        pool_q.push_back(
            token_adjacencies
                .get(&src)
                .unwrap()
                .iter()
                .collect::<VecDeque<_>>(),
        );

        // XXX: should be able to parallelize this easily if needed
        loop {
            let pool = match pool_q.pop_back() {
                Some(p) if p.is_empty() => {
                    // if our p here is empty we are done with this level of the graph, so:
                    // - backtrack
                    pool_trail.pop();
                    // - and reset the token we are looking for
                    if pool_trail.is_empty() {
                        src = start.clone();
                    } else {
                        (_, src) = if let Some((l, r)) = get_tokens(start, &pool_trail, pool_map) {
                            (l, r)
                        } else {
                            // XXX: wat do
                            error!("could not recontstruct token path for {:?}", pool_trail);
                            return vec![];
                        };
                    }
                    continue;
                },
                Some(mut p) => {
                    // we have more to explore on this level
                    let ret = p.pop_back().unwrap();
                    pool_q.push_back(p);
                    ret
                },
                None => break, // queue is empty, we are done
            };

            // if we already have this pool in our path, just ignore it.
            if pool_trail.contains(pool) {
                continue;
            }

            dst = pool_map.get(pool).unwrap().ask_token(&src);

            // If the new destination is our starting point, then we found a cycle and
            // we can push it to the valid trade paths.
            if dst == *start && !pool_trail.is_empty() {
                let mut p: Vec<Addr> = pool_trail.clone().iter().map(|e| (*e).clone()).collect();
                p.push((*pool).clone());
                pool_paths.push(p);

                continue;
            }

            // If further exploration pushes us above cycle length, then instead of going
            // deeper, see if there is another pool at this level, that gets us to our goal.
            // i.e. just ignore the current pool and see if the next == start.
            if pool_trail.len() + 1 > MAX_PATH_LEN - 1 {
                continue;
            }

            pool_q.push_back(
                token_adjacencies
                    .get(&dst)
                    .unwrap()
                    .iter()
                    .filter(|p| *p != pool)
                    .collect::<VecDeque<_>>(),
            );
            pool_trail.insert(pool);
            src = dst;
        }

        pool_paths
    }

    fn prepare_paths<'a>(
        &mut self,
        start: Token,
        pools: &'a dex::Pools,
    ) -> HashMap<Addr, &'a Pool> {
        // lets us efficiently look up pools
        let mut pool_map: PoolMap<'_> = HashMap::new();
        // token -> pools
        let mut token_adjacencies: TokenAdjacencyMap = HashMap::new();

        // setup all the edges
        for pool in pools.0.iter() {
            let p = pool.token_pair();

            let left = p.0[0].clone();
            let right = p.0[1].clone();

            token_adjacencies
                .entry(left.clone())
                .or_insert_with(HashSet::new)
                .insert(pool.contract_address.clone());
            token_adjacencies
                .entry(right.clone())
                .or_insert_with(HashSet::new)
                .insert(pool.contract_address.clone());

            pool_map.insert(pool.contract_address.clone(), pool);
        }

        // XXX: do I care about cleaning up the graph to remove unused pairs?
        // pg.retain(|_, v| v.len() > 0);
        // token_adjacencies.retain(|_, v| v.len() > 1);

        self.paths = self.dfs(&start, &pool_map, &token_adjacencies);
        debug!("pool_paths {:#?}", self.paths.len());
        pool_map
    }
}

#[async_trait::async_trait]
impl Strategy for MultiHop {
    async fn execute(
        &mut self,
        _height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools,
    ) -> Vec<Trade> {
        let mut trades = vec![];

        let start = dex::Token::ust();

        let pool_map;
        if self.num_pools == pools.0.len() {
            // No need to rebuild paths, just update liq
            debug!("num_pools unchanged, no need to build graph");
            pool_map = HashMap::from_iter(pools.0.iter().map(|p| (p.contract_address.clone(), p)));
        } else {
            self.num_pools = pools.0.len();
            pool_map = self.prepare_paths(start.clone(), pools);
        }

        for path in &self.paths {
            // debug!("path: {:#?}", path);
            let ppath = path
                .iter()
                .map(|a| *pool_map.get(a).unwrap())
                .collect_vec()
                .into_iter();
            if let Some(liq) = compute_cycle_liq(&start, ppath.clone()) {
                // debug!("got a good trade {} on {:?}", liq, path);
                let bal: Decimal256 = *funds.get(&start).unwrap_or(&Decimal256::zero());
                let size = if liq > bal { bal } else { liq };

                // let size = liq;
                // XXX: the mismatch between size and our balance should be captured in metrics
                if let Some((expected_profit, messages, route)) =
                    trade_path(sender, ppath.clone(), &start, size)
                {
                    trades.push(Trade {
                        size,
                        expected_profit,
                        messages,
                        route,
                    });
                }
            }
        }

        trades.sort_by(|a, b| b.expected_profit.cmp(&a.expected_profit));
        let (mut filter, mut out) = if !trades.is_empty() {
            let a = trades.first().unwrap().route.clone();
            (
                HashSet::from_iter(a.into_iter()),
                vec![trades.first().unwrap().clone()],
            )
        } else {
            (HashSet::new(), vec![])
        };

        // make sure that trades do not share any pools
        debug!("trade len {:?}", trades.len());
        for t in trades.iter().skip(1) {
            let ft = HashSet::from_iter(t.route.clone().into_iter());

            if filter.is_disjoint(&ft) {
                out.push(t.clone());
                for e in ft {
                    filter.insert(e);
                }
            }
        }
        debug!("out len {:?}", out.len());
        out
    }
}

fn get_tokens(
    start: &Token,
    trail: &XxIndexSet<&Addr>,
    pool_map: &HashMap<Addr, &Pool>,
) -> Option<(Token, Token)> {
    if trail.is_empty() {
        return None;
    }

    let mut src = start.clone();
    let mut dst = start.clone();

    for p in trail {
        src = dst;
        dst = pool_map.get(p)?.ask_token(&src);
    }

    Some((src, dst))
}
