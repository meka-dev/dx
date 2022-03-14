use bigint::U256;
use common::tracing::trace;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Decimal256 as StdDecimal256, Fraction, Uint256 as StdUint256};
use terrars::{
    dex::{belief_price, max_spread, Pool, Token},
    AccountId, MsgExecuteContract,
};

use crate::strategy::Route;

// Iterate through a bunch of pools and properly arrange them s.t.
// asset0 is always bid and asset1 is the ask.
// XXX: do we actually want to implement this here?
pub fn trade_path<'a, T: Iterator<Item = &'a Pool>>(
    sender: &AccountId,
    pools: T,
    start: &Token,
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
) -> Option<Decimal256> {
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
