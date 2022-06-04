use bigint::U256;
use common::tracing::trace;
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Decimal256 as StdDecimal256, Fraction, Uint256 as StdUint256};

use crate::{Pool, Token};

pub fn compute_cycle_liq<'a, T: Iterator<Item = &'a Pool>>(
    start: &Token,
    mut pools: T,
) -> Option<Decimal256> {
    let (mut a, mut a_p, mut next_bid_token) = match pools.next() {
        Some(Pool {
            assets: [left, right],
            ..
        }) if left.token == *start => (left.amount, right.amount, right.token.clone()),
        Some(Pool {
            assets: [left, right],
            ..
        }) if right.token == *start => (right.amount, left.amount, left.token.clone()),
        _ => return None,
    };

    // We accumulate (in a and a_p) the liquidity along the given path, taking fees
    // (r1, r2) into account. r1 for the bid and r2 for the ask.
    let mut r1 = Decimal256::one();
    let mut r2 = Decimal256::one();
    for pool in pools {
        r1 = Decimal256::one() - pool.swap_fee_bid;
        r2 = Decimal256::one() - pool.swap_fee_ask;

        let (p1_bid, p1_ask) = match &pool.assets {
            [left, right] if left.token == next_bid_token => (left, right),
            [left, right] if right.token == next_bid_token => (right, left),
            _ => return None,
        };

        trace!(
            "p0 bid amt {:}, p0_ask amt {:}, p1_bid amt {:}, p1_ask amt {:}",
            a,
            a_p,
            p1_bid.amount,
            p1_ask.amount,
        );

        a = (a * p1_bid.amount) / (p1_bid.amount + r1 * r2 * a_p);
        a_p = (a_p * p1_ask.amount * r2) / (p1_bid.amount + r1 * r2 * a_p);

        next_bid_token = p1_ask.token.clone();
        trace!("next_bid_token {:?}", next_bid_token);
    }

    // And then compute the optimal swap amount
    let optimal = sd256_to_d256(d256_to_sd256(r1 * r2 * a * a_p).sqrt());

    trace!("op {:}, a {:?} a_p {:?}", optimal, a, a_p);
    if optimal < a {
        return None;
    }

    Some(optimal - a)
    // Some((optimal.0 / 10u64.pow(18 - decimals as u32).into()).into())
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
