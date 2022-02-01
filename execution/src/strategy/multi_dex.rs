use std::str::FromStr;

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Decimal256 as StdDecimal256, Uint128};
use itertools::Itertools;

use crate::dex;
use crate::strategy::Strategy;

#[derive(Default)]
pub struct SinglePair {}

fn compute_cycle_liq(start: &dex::Token, pools: &[&Box<dyn dex::Pool>]) -> Option<Uint128> {
    if pools.len() != 2 {
        // XXX: generalize
        return None;
    }

    let r2 = Decimal256::from_str("0.997").unwrap();

    let [p0_bid, p0_ask] = if pools[0].coins()[0].token == *start {
        pools[0].coins()
    } else {
        let mut ass = pools[0].coins();
        ass.reverse();
        ass
    };

    // XXX: assume to start with ust bid
    // XXX: this also matches stable<>stable pairs
    // XXX: filter out stableswap pairs
    let [p1_bid, p1_ask] = if pools[1].coins()[0].token == p0_ask.token {
        pools[1].coins()
    } else {
        let mut ass = pools[1].coins().clone();
        ass.reverse();
        ass
    };

    log::debug!(
        "p0 bid {:}, p0_ask {:}, p1_bid {:}, p1_ask {:}",
        p0_bid.amount,
        p0_ask.amount,
        p1_bid.amount,
        p1_ask.amount
    );
    // XXX: newer versions of decimal have a `from_atomics()`
    log::debug!(
        "p0 bid amt {:}, p0_ask amt {:}, p1_bid amt {:}, p1_ask amt {:}",
        p0_bid.amount_decimal,
        p0_ask.amount_decimal,
        p1_bid.amount_decimal,
        p1_ask.amount_decimal
    );

    let a = (p0_bid.amount_decimal * p1_bid.amount_decimal)
        / (p1_bid.amount_decimal + r2 * p0_ask.amount_decimal);
    let a_p = (p0_ask.amount_decimal * p1_ask.amount_decimal * r2)
        / (p1_bid.amount_decimal + r2 * p0_ask.amount_decimal);

    // ??????
    let op_std = StdDecimal256::from_str(&(r2 * a * a_p).to_string())
        .unwrap()
        .sqrt();
    let op = Decimal256::from_str(&op_std.to_string()).unwrap();

    if op < a {
        return None;
    }

    // scale back to 6 decimals
    let scaled: Uint256 = ((op - a) / Decimal256::from_uint256(10u128.pow(12)))
        .0
        .into();
    Some(scaled.into())
}

impl Strategy for SinglePair {
    fn execute(&self, pools: &dex::Pools) -> Vec<terrars::Message> {
        // crazy arbs go here
        let start = dex::Token::Native {
            denom: "uusd".to_string(),
        };
        for (token, pairs) in pools.index_by_token() {
            // We're only looking at cross dex two hop arbs, i.e. take all two pair
            // permutations for any token, eg ust -> luna on astroport and luna -> ust
            // on terraswap
            if token.is_native() && !token.is_luna() {
                continue;
            }

            // XXX: Clone is not implemented for Box<dyn Pool>, so we need to store refs,
            //      but then we end up with Vec<&Box<dyn Pool>> which is not Pools
            for ps in pairs.iter().map(|i| &pools.0[*i]).permutations(2) {
                if let Some(liq) = compute_cycle_liq(&start, &ps) {
                    log::info!(
                        "got a good trade {} { }{:?} to {} {:?}",
                        liq,
                        ps[0].addr(),
                        ps[0].token_pair(),
                        ps[1].addr(),
                        ps[1].token_pair()
                    );
                    // dex::Pools::from(ps).trade_path(&start, liq); // XXX
                }
            }
        }

        // Empty list for now.
        vec![]
    }
}
