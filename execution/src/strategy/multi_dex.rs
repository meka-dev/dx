use std::str::FromStr;

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Decimal256 as StdDecimal256, Uint128};
use itertools::Itertools;

use crate::dex;
use crate::strategy::Strategy;

#[derive(Default)]
pub struct SinglePair {}

fn compute_cycle_liq(
    pools: &Vec<&Box<dyn dex::Pool>>,
    tokens: &dex::TokenCache,
) -> Option<Uint128> {
    if pools.len() != 2 {
        return None;
    }

    let r2 = Decimal256::from_str("0.997").unwrap();

    let [p0_ask, p0_offer] = if pools[0].coins()[0].token.is_ust() {
        let mut ass = pools[0].coins();
        ass.reverse();
        ass
    } else {
        pools[0].coins()
    };

    // XXX: assume to start with ust offer
    // XXX: this also matches stable<>stable pairs
    // XXX: filter out stableswap pairs
    let tok_decimals = match p0_ask.token {
        dex::Token::Cw20 { contract_addr: a } => tokens.0[&a].decimals,
        dex::Token::Native { denom: d } if d == "uusd" => 6,
        _ => {
            log::error!("no decimals {:?} {:?}", p0_ask.token, p0_offer.token);
            return None;
        },
    };

    let [p1_ask, p1_offer] = if pools[1].coins()[0].token.is_ust() {
        pools[1].coins()
    } else {
        let mut ass = pools[1].coins().clone();
        ass.reverse();
        ass
    };

    log::debug!(
        "p0 offer {:}, p0_ask {:}, p1_offer {:}, p1_ask {:}",
        p0_offer.amount,
        p0_ask.amount,
        p1_offer.amount,
        p1_ask.amount
    );
    // XXX: newer versions of decimal have a `from_atomics()`
    let p0_offer_amt: Decimal256 =
        Decimal256::from_uint256(p0_offer.amount.u128()) / Decimal256::from_uint256(10u128.pow(6));
    let p0_ask_amt: Decimal256 = Decimal256::from_uint256(p0_ask.amount.u128())
        / Decimal256::from_uint256(10u128.pow(tok_decimals as u32));
    let p1_offer_amt: Decimal256 = Decimal256::from_uint256(p1_offer.amount.u128())
        / Decimal256::from_uint256(10u128.pow(tok_decimals as u32));
    let p1_ask_amt: Decimal256 =
        Decimal256::from_uint256(p1_ask.amount.u128()) / Decimal256::from_uint256(10u128.pow(6));
    log::debug!(
        "p0 offer amt {:}, p0_ask amt {:}, p1_offer amt {:}, p1_ask amt {:}",
        p0_offer_amt,
        p0_ask_amt,
        p1_offer_amt,
        p1_ask_amt
    );

    let a: Decimal256 = (p0_offer_amt * p1_offer_amt) / (p1_offer_amt + r2 * p0_ask_amt);
    let a_p: Decimal256 = (p0_ask_amt * p1_ask_amt * r2) / (p1_offer_amt + r2 * p0_ask_amt);

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
    fn execute(&self, pools: &dex::Pools, tokens: &dex::TokenCache) -> Vec<terrars::Message> {
        // crazy arbs go here
        for (asset, pairs) in pools.index_by_token() {
            // We're only looking at cross dex two hop arbs, i.e. take all two pair
            // permutations for any token, eg ust -> luna on astroport and luna -> ust
            // on terraswap
            if !asset.is_ust() {
                continue;
            }

            for ps in pairs.iter().map(|i| &pools.0[*i]).permutations(2) {
                if let Some(liq) = compute_cycle_liq(&ps, tokens) {
                    log::info!(
                        "got a good trade {} {:?} to {:?}",
                        liq,
                        ps[0].token_pair(),
                        ps[1].token_pair()
                    )
                }
            }
        }

        // Empty list for now.
        vec![]
    }
}
