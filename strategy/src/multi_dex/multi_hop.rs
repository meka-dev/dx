#![allow(clippy::borrowed_box)]

use std::collections::{HashMap, HashSet, VecDeque};

use common::tracing::{debug, error, trace};
use cosmwasm_bignumber::Decimal256;
use indexmap::IndexSet;
use itertools::Itertools;
use tendermint::block;
use xxhash_rust::xxh3::Xxh3Builder;

use crate::{
    metrics,
    xyk::compute_cycle_liq,
    Leg,
    Pool,
    PoolId,
    Pools,
    Route,
    Strategy,
    Token,
    Trade,
};

// type XxIndexMap<K, V> = IndexMap<K, V, Xxh3Builder>;
type XxIndexSet<K> = IndexSet<K, Xxh3Builder>;

// Adjacency map for token -> pools
type TokenAdjacencyMap = HashMap<Token, HashSet<PoolId>>;

// Mapping to allow efficient pool data look-up
type PoolMap = HashMap<PoolId, Pool>;

#[derive(Default)]
pub struct MultiHop {
    num_pools: usize,
    sync_height: u64,
    funds: Decimal256,
    start: Token,
    pools: HashMap<PoolId, Pool>,
    paths: Vec<Vec<PoolId>>,
    pool_paths: HashMap<PoolId, Vec<Vec<PoolId>>>,
}

impl metrics::Labeled for MultiHop {
    const MODULE: metrics::Module = metrics::Module::MultiDex;
    const NAME: metrics::Name = metrics::Name::MultiHop;
}

const MAX_PATH_LEN: usize = 4;

impl MultiHop {
    pub fn new(start: Token) -> Self {
        MultiHop {
            start,
            ..MultiHop::default()
        }
    }

    fn dfs(
        &self,
        start: &Token,
        pool_map: &PoolMap,
        token_adjacencies: &TokenAdjacencyMap,
    ) -> Vec<Vec<PoolId>> {
        let mut pool_paths: Vec<Vec<PoolId>> = vec![];
        let mut pool_trail: XxIndexSet<&PoolId> = XxIndexSet::with_hasher(Xxh3Builder::new());
        // a queue for every level/node in the tree, e.g. a queue of queues
        let mut pool_q: VecDeque<VecDeque<&PoolId>> = VecDeque::new();

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

            dst = pool_map.get(pool).unwrap().ask_asset(&src).unwrap();

            // If the new destination is our starting point, then we found a cycle and
            // we can push it to the valid trade paths.
            if dst == *start && !pool_trail.is_empty() {
                let mut p: Vec<PoolId> = pool_trail.clone().iter().map(|e| (*e).clone()).collect();
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

    fn prepare_paths(&mut self, start: &Token, pools: &Pools) -> PoolMap {
        // lets us efficiently look up pools
        let mut pool_map: PoolMap = HashMap::new();
        // token -> pools
        let mut token_adjacencies: TokenAdjacencyMap = HashMap::new();

        // setup all the edges
        for pool in pools.iter() {
            let p = pool.assets.clone();

            let left = p[0].token.clone();
            let right = p[1].token.clone();

            token_adjacencies
                .entry(left.clone())
                .or_insert_with(HashSet::new)
                .insert(pool.id.clone());
            token_adjacencies
                .entry(right.clone())
                .or_insert_with(HashSet::new)
                .insert(pool.id.clone());

            pool_map.insert(pool.id.clone(), pool.clone());
        }

        // XXX: do I care about cleaning up the graph to remove unused pairs?
        // pg.retain(|_, v| v.len() > 0);
        // token_adjacencies.retain(|_, v| v.len() > 1);

        self.paths = self.dfs(start, &pool_map, &token_adjacencies);
        let mut pps: HashMap<PoolId, Vec<Vec<PoolId>>> = HashMap::new();
        for path in &self.paths {
            for step in path {
                // pps.entry(step.clone()).or_insert(XxIndexSet::with_hasher(Xxh3Builder::new())).
                // insert(path.clone());
                pps.entry(step.clone()).or_insert(vec![]).push(path.clone());
            }
        }
        self.pool_paths = pps;
        debug!("pool_paths {:#?}", self.paths.len());
        pool_map
    }
}

#[async_trait::async_trait]
impl Strategy for MultiHop {
    #[allow(clippy::needless_collect)]
    async fn execute_mempool_tx(&mut self, height: block::Height, pools: &Pools) -> Vec<Trade> {
        if self.num_pools == 0 || self.sync_height < height.into() {
            // pools need to be loaded by block sync
            return vec![];
        }

        let mut touched: Vec<PoolId> = vec![];
        {
            for ps in pools {
                if let Some(p) = self.pools.get_mut(&ps.id) {
                    *p = (*ps).clone();
                    touched.push(ps.id.clone());
                }
            }
        }

        let paths = HashSet::<Vec<PoolId>>::from_iter(
            touched
                .iter()
                .flat_map(|t| (*self.pool_paths.get(t).unwrap_or(&vec![])).clone()),
        );
        // error!("paths {:?}", paths.len());
        let mut trades = paths
            .iter()
            .filter_map(|path| find_trade(path, &self.pools, &self.start, self.funds))
            .collect::<Vec<_>>();

        trades.sort_by(|a, b| b.expected_profit.cmp(&a.expected_profit));
        trades.into_iter().take(1).collect_vec()
    }

    async fn execute_pools(
        &mut self,
        height: block::Height,
        funds: Decimal256,
        pools: &Pools,
    ) -> Vec<Trade> {
        let p = if self.num_pools == pools.len() {
            // No need to rebuild paths, just update liq
            debug!("num_pools unchanged, no need to build graph");
            HashMap::from_iter(pools.iter().map(|p| (p.id.clone(), (*p).clone())))
        } else {
            self.num_pools = pools.len();
            self.prepare_paths(&self.start.clone(), pools)
        };

        self.pools = p;
        self.funds = funds;
        self.sync_height = height.into();

        // if we can react to all tx via the mempool then we dont need the following anymore and
        // that also means less lock contention
        // let bal: Uint256 = *funds.get(&start).unwrap_or(&Uint256::zero());
        // if bal <= 1000000u128.into() {
        // return vec![];
        // }
        // let spools = self.pools.read().unwrap();
        // let mut trades = self
        // .paths
        // .par_iter()
        // .filter_map(|path| find_trade(path, &spools, &start, sender, bal))
        // .collect::<Vec<_>>();
        //
        // trades.sort_by(|a, b| b.expected_profit.cmp(&a.expected_profit));
        // trades.into_iter().take(1).collect_vec()
        return vec![];
    }
}

// Check if a given path can generate a profitable trade (ignoring tx fee)
fn find_trade(
    path: &[PoolId],
    spools: &HashMap<PoolId, Pool>,
    start: &Token,
    bal: Decimal256,
) -> Option<Trade> {
    // debug!("path: {:#?}", path);
    let ppath = path
        .iter()
        .map(|a| spools.get(a).unwrap())
        .collect_vec()
        .into_iter();
    if let Some(liq) = compute_cycle_liq(start, ppath.clone()) {
        // debug!("got a good trade {} on {:?}", liq, path);
        let size = if liq > bal { bal } else { liq };

        // XXX: the mismatch between size and our balance should be captured in metrics
        if let Some((expected_profit, legs, route)) = trade_path(ppath, start, size) {
            return Some(Trade {
                size,
                expected_profit,
                legs,
                route,
            });
        }
    }
    None
}

// Iterate through a bunch of pools and properly arrange them s.t.
// asset0 is always bid and asset1 is the ask.
pub fn trade_path<'a, T: Iterator<Item = &'a Pool>>(
    pools: T,
    start: &Token,
    amount: Decimal256,
) -> Option<(Decimal256, Vec<Leg>, Route)> {
    let mut cb = amount;
    let mut bid_token = start.clone();
    let mut legs = vec![];
    let mut route: Vec<PoolId> = vec![];

    for p in pools {
        let ask_token = p.ask_asset(&bid_token)?;
        if let Some(swap_ret) = p.swap(&bid_token, cb) {
            if swap_ret.is_zero() {
                return None;
            }

            trace!(
                "swap {} {:?} for {} {:?} on {}",
                cb,
                bid_token,
                swap_ret,
                ask_token,
                p.id,
            );
            route.push(p.id.clone());
            legs.push(Leg {
                in_token: bid_token.clone(),
                in_amount: cb,
                min_out_amount: Some(cb),
                pool: p.clone(),
            });

            cb = swap_ret;
            bid_token = ask_token;
        } else {
            trace!("could not swap on {}", p.id);
            return None;
        }
    }

    trace!("started with {}, ended with {}", amount, cb);
    if cb <= amount {
        return None;
    }
    trace!("positive delta {}", cb - amount);

    Some((cb - amount, legs, route))
}

fn get_tokens(
    start: &Token,
    trail: &XxIndexSet<&PoolId>,
    pool_map: &PoolMap,
) -> Option<(Token, Token)> {
    if trail.is_empty() {
        return None;
    }

    let mut src = start.clone();
    let mut dst = start.clone();

    for p in trail {
        src = dst;
        dst = pool_map.get(*p)?.ask_asset(&src)?;
    }

    Some((src, dst))
}
