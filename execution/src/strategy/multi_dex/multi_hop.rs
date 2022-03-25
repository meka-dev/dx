#![allow(clippy::borrowed_box)]

use std::collections::{HashMap, HashSet, VecDeque};

use bigint::U256;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Addr, Decimal256 as StdDecimal256, Fraction, Uint256 as StdUint256};
use indexmap::IndexSet;
use itertools::Itertools;
use rayon::prelude::*;
use tendermint::block;
use xxhash_rust::xxh3::Xxh3Builder;

use common::tracing::{debug, error, trace};

use terrars::{
    dex::{self, belief_price, max_spread, Pool, Token},
    AccountId, MsgExecuteContract,
};

use crate::strategy::Route;

use crate::{
    accounts,
    strategy::{metrics, Strategy, Trade},
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

        let mut trades = self
            .paths
            .par_iter()
            .filter_map(|path| {
                // debug!("path: {:#?}", path);
                let ppath = path
                    .iter()
                    .map(|a| *pool_map.get(a).unwrap())
                    .collect_vec()
                    .into_iter();
                if let Some(liq) = compute_cycle_liq(&start, ppath.clone()) {
                    // debug!("got a good trade {} on {:?}", liq, path);
                    let bal: Uint256 = *funds.get(&start).unwrap_or(&Uint256::zero());
                    let size = if liq > bal { bal } else { liq };

                    // let size = liq;
                    // XXX: the mismatch between size and our balance should be captured in metrics
                    if let Some((expected_profit, messages, route)) =
                        trade_path(sender, ppath.clone(), &start, size)
                    {
                        return Some(Trade {
                            size,
                            expected_profit,
                            messages,
                            route,
                        });
                    }
                }
                None
            })
            .collect::<Vec<_>>();

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

// Iterate through a bunch of pools and properly arrange them s.t.
// asset0 is always bid and asset1 is the ask.
pub fn trade_path<'a, T: Iterator<Item = &'a Pool>>(
    sender: &AccountId,
    pools: T,
    start: &Token,
    amount: Uint256,
) -> Option<(Uint256, Vec<MsgExecuteContract>, Route)> {
    let mut msgs = vec![];
    let mut cb = amount;
    let mut bid_token = start.clone();
    let mut route = vec![];

    for p in pools {
        let ask_token = p.ask_token(&bid_token);
        if let Some((swap_ret, spread)) = p.compute_swap(&bid_token, &ask_token, cb) {
            if swap_ret.is_zero() || spread.is_zero() {
                return None;
            }

            trace!(
                "swap {} {:?} for {} {:?} with a spread of {} using max_spread {} on {}",
                cb,
                bid_token,
                swap_ret,
                ask_token,
                spread,
                max_spread(swap_ret, spread),
                p.contract_address,
            );
            msgs.push(p.swap(
                sender,
                &bid_token,
                cb,
                belief_price(cb, swap_ret),
                max_spread(swap_ret, spread),
            ));
            route.push(p.contract_address.clone());

            cb = swap_ret;
            bid_token = ask_token;
        } else {
            trace!("could not swap on {}", p.contract_address);
            return None;
        }
    }

    trace!("started with {}, ended with {}", amount, cb);
    if cb <= amount {
        return None;
    }
    trace!("positive delta {}", cb - amount);

    Some((cb - amount, msgs, route))
}

// XXX: this might eventually go into a contract
pub fn compute_cycle_liq<'a, T: Iterator<Item = &'a Pool>>(
    start: &Token,
    mut pools: T,
) -> Option<Uint256> {
    let r2 = Decimal256::permille(997); // XXX: get fee from chain

    let (mut a, mut a_p, mut next_bid_token, decimals) = match pools.next() {
        Some(Pool {
            assets: [left, right],
            ..
        }) if left.token == *start => (
            left.amount_decimal,
            right.amount_decimal,
            right.token.clone(),
            left.decimals,
        ),
        Some(Pool {
            assets: [left, right],
            ..
        }) if right.token == *start => (
            right.amount_decimal,
            left.amount_decimal,
            left.token.clone(),
            right.decimals,
        ),
        _ => return None,
    };

    // We accumulate (in a and a_p) the liquidity along the given path, taking fees
    // (r1, r2) into account.
    for pool in pools {
        let (p1_bid, p1_ask) = match &pool.assets {
            [left, right] if left.token == next_bid_token => (left, right),
            [left, right] if right.token == next_bid_token => (right, left),
            _ => return None,
        };

        // XXX: test with different decimal values
        if p1_ask.decimals != decimals {
            return None;
        }

        trace!(
            "p0 bid amt {:}, p0_ask amt {:}, p1_bid amt {:}, p1_ask amt {:}",
            a,
            a_p,
            p1_bid.amount_decimal,
            p1_ask.amount_decimal
        );

        a = (a * p1_bid.amount_decimal) / (p1_bid.amount_decimal + r2 * a_p);
        a_p = (a_p * p1_ask.amount_decimal * r2) / (p1_bid.amount_decimal + r2 * a_p);

        next_bid_token = p1_ask.token.clone();
        trace!("next_bid_token {:?}", next_bid_token);
    }

    // And then compute the optimal swap amount
    let mut optimal = sd256_to_d256(d256_to_sd256(r2 * a * a_p).sqrt());

    trace!("op {:}, a {:?} a_p {:?}", optimal, a, a_p);
    if optimal < a {
        return None;
    }

    optimal = optimal - a;
    let t: Decimal256 = Decimal256::from_ratio(
        Uint256::one(),
        Into::<Uint256>::into(10u128.pow(decimals as u32)),
    );
    if optimal < t {
        // If we are getting out less than the precision of the token then we just
        // return None
        return None;
    }
    Some((optimal.0 / 10u64.pow(18 - decimals as u32).into()).into())
}

fn d256_to_sd256(d: Decimal256) -> StdDecimal256 {
    let mut buf = [0u8; 32];
    d.0.to_little_endian(buf.as_mut_slice());

    StdDecimal256::from_ratio(
        StdUint256::from_le_bytes(buf),
        StdUint256::from(10u128.pow(18)),
    )
}

fn sd256_to_d256(d: StdDecimal256) -> Decimal256 {
    Decimal256(U256::from_little_endian(&d.numerator().to_le_bytes()))
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
