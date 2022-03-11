#![allow(clippy::borrowed_box)]

use std::str::FromStr as _;

use bigint::U256;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Decimal256 as StdDecimal256, Fraction, Uint256 as StdUint256};
use itertools::Itertools;
use tendermint::block;

use common::tracing::debug;
use terrars::{
    dex::{self, Token},
    AccountId, MsgExecuteContract,
};

use crate::{
    accounts,
    strategy::{metrics, Route, Strategy, Trade},
};

#[derive(Default)]
pub struct SinglePair {}

// Iterate through a bunch of pools and properly arrange them s.t.
// asset0 is always bid and asset1 is the ask.
// XXX: do we actually want to implement this here?
fn trade_path(
    sender: &AccountId,
    pools: &[&dex::Pool; 2],
    start: &dex::Token,
    amount: Decimal256,
) -> Option<(Decimal256, Vec<MsgExecuteContract>, Route)> {
    let mut msgs = vec![];
    let mut cb = amount;
    let mut bid_token = start.clone();
    let mut route = vec![];

    for p in pools {
        let ask_token = p.ask_token(&bid_token);
        if let Some((swap_ret, spread)) = p.compute_swap(&bid_token, &ask_token, cb) {
            if swap_ret == Decimal256::zero() || spread == Decimal256::zero() {
                return None;
            }

            debug!(
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
            debug!("could not swap on {}", p.contract_address);
            return None;
        }
    }

    debug!("started with {}, ended with {}", amount, cb);
    if cb <= amount {
        return None;
    }
    debug!("positive delta {}", cb - amount);

    Some((cb - amount, msgs, route))
}

// only need this for the actual msg
fn belief_price(amount_in: Decimal256, amount_out: Decimal256) -> Decimal256 {
    amount_in / amount_out
}

// XXX: astroport has an upper limit on max_spread of 0.5
fn max_spread(_amount_out: Decimal256, _spread: Decimal256) -> Decimal256 {
    // spread / amount_out
    Decimal256::zero()
}

// XXX: this might eventually go into a contract
fn compute_cycle_liq<'a, T: Iterator<Item = &'a dex::Pool>>(
    start: &dex::Token,
    mut pools: T,
) -> Option<Decimal256> {
    let r2 = Decimal256::from_str("0.997").unwrap(); // XXX: get fee from chain

    let (mut a, mut a_p, mut next_bid_token, decimals) = match pools.next() {
        Some(dex::Pool {
            assets: [left, right],
            ..
        }) if left.token == *start => (
            left.amount_decimal,
            right.amount_decimal,
            right.token.clone(),
            left.decimals,
        ),
        Some(dex::Pool {
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
        let [p1_bid, p1_ask] = if pool.assets[0].token == next_bid_token {
            pool.assets.clone()
        } else {
            let mut ass = pool.assets.clone();
            ass.reverse();
            ass
        };

        // XXX: test with different decimal values
        if p1_ask.decimals != decimals {
            return None;
        }

        debug!(
            "p0 bid amt {:}, p0_ask amt {:}, p1_bid amt {:}, p1_ask amt {:}",
            a, a_p, p1_bid.amount_decimal, p1_ask.amount_decimal
        );

        a = (a * p1_bid.amount_decimal) / (p1_bid.amount_decimal + r2 * a_p);
        a_p = (a_p * p1_ask.amount_decimal * r2) / (p1_bid.amount_decimal + r2 * a_p);

        next_bid_token = p1_ask.token.clone();
    }

    // And then compute the optimal swap amount
    let mut optimal = sd256_to_d256(d256_to_sd256(r2 * a * a_p).sqrt());

    debug!("op {:}", optimal);
    debug!("a {:}", a);
    debug!("a_p {:}", a_p);
    if optimal < a {
        return None;
    }

    optimal = optimal - a;
    let t: Decimal256 = Decimal256::from_ratio(
        Uint256::one(),
        Into::<Uint256>::into(10u128.pow(decimals as u32)),
    );
    debug!("optimal {:}", optimal);
    if optimal < t {
        debug!("compute_swap below threshold");
        // If we are getting out less than the precision of the token then we just
        // return None
        return None;
    }
    Some(optimal)
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

impl metrics::Labeled for SinglePair {
    const MODULE: metrics::Module = metrics::Module::MultiDex;
    const NAME: metrics::Name = metrics::Name::SinglePair;
}

// XXX: make sure to properly calculate tax
#[async_trait::async_trait]
impl Strategy for SinglePair {
    async fn execute(
        &mut self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools,
    ) -> Vec<Trade> {
        let mut trades = vec![];

        // crazy arbs go here
        let start = dex::Token::ust();

        debug!("#{}", height);
        for (token, pairs) in pools.index_by_token(&Token::ust()) {
            // We're only looking at cross dex two hop arbs, i.e. take all two pair
            // permutations for any token, eg ust -> luna on astroport and luna -> ust
            // on terraswap
            if token.is_native() && !token.is_luna() {
                continue;
            }

            for indices in pairs.iter().permutations(2) {
                if indices.len() != 2 {
                    break;
                }

                let (left, right) = (&pools.0[*(indices[0])], &pools.0[*(indices[1])]);
                debug!(
                    "compute liq for path left {:} right {:}",
                    left.contract_address, right.contract_address
                );

                if let Some(liq) = compute_cycle_liq(&start, [left, right].into_iter()) {
                    debug!(
                        "got a good trade {} {} to {}",
                        liq, left.contract_address, right.contract_address,
                    );
                    // Check if we have sufficient funds available to execute the optimal trade.
                    // If not, then we just trade max and hope we can still pull enough of a
                    // profit.
                    // We could also consider adding logic to spread the arb across multiple tx.
                    // XXX: this still does not take gas fees into account
                    let bal: Decimal256 = *funds.get(&start).unwrap_or(&Decimal256::zero());
                    let _size = if liq > bal { bal } else { liq };
                    let size = liq;
                    // XXX: the mismatch between size and our balance should be captured in metrics
                    if let Some((expected_profit, messages, route)) =
                        trade_path(sender, &[left, right], &start, size)
                    {
                        debug!("{:?}", messages);
                        trades.push(Trade {
                            size,
                            expected_profit,
                            messages,
                            route,
                        });
                    } else {
                        debug!(
                            "but could not seize the opportunity with my balance {:}",
                            bal
                        );
                    }
                }
            }
        }

        trades.sort_by(|a, b| b.expected_profit.cmp(&a.expected_profit));
        trades
    }
}
