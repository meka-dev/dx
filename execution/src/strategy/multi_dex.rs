#![allow(clippy::borrowed_box)]

use std::str::FromStr;

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{
    coins, to_binary, CosmosMsg, Decimal, Decimal256 as StdDecimal256, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use itertools::Itertools;
use terraswap::{
    asset::{Asset, AssetInfo},
    pair::{Cw20HookMsg, ExecuteMsg},
};

use crate::{dex, strategy::Strategy};

#[derive(Default)]
pub struct SinglePair {}

// Iterate through a bunch of pools and properly arrange them s.t.
// asset0 is always bid and asset1 is the ask.
// XXX: do we actually want to implement this here?
fn trade_path(
    pools: &[&Box<dyn dex::Pool>; 2],
    start: &dex::Token,
    amount: Uint128,
) -> Option<(Uint128, Vec<CosmosMsg>)> {
    let mut msgs: Vec<CosmosMsg> = vec![];
    let mut cb = amount;
    let mut bid_token = start.clone();

    for p in pools {
        let ask_token = p.ask_token(&bid_token);
        if let Some((r, s)) = p.compute_swap(&bid_token, &ask_token, cb) {
            log::info!(
                "swap {} {:?} for {} {:?} with a spread of {} on {}",
                cb,
                bid_token,
                r,
                ask_token,
                s,
                p.addr(),
            );
            msgs.push(make_trade_msg(&bid_token, *p, r, belief_price(cb, r))?);

            cb = r;
            bid_token = ask_token;
        } else {
            log::info!("could not swap on {}", p.addr());
            return None;
        }
    }

    log::info!("started with {}, ended with {}", amount, cb);
    if cb <= amount {
        return None;
    }
    log::info!("positive delta {}", cb - amount);

    Some((cb - amount, msgs))
}

// only need this for the actual msg
fn belief_price(amount_in: Uint128, amount_out: Uint128) -> Decimal {
    Decimal::from_ratio(amount_in, amount_out)
}

// token - bid/offer token
// native -> token goes as swap via pair contract
// token -> native goes as send via token contract
fn make_trade_msg(
    token: &dex::Token,
    pool: &Box<dyn dex::Pool>,
    amount: Uint128,
    belief_price: Decimal,
) -> Option<CosmosMsg> {
    match token {
        dex::Token::Cw20 {
            contract_addr: addr,
        } => {
            // token -> native
            let s = to_binary(&Cw20HookMsg::Swap {
                to: None, // to is set to sender of initial msg
                belief_price: Some(belief_price),
                max_spread: None, // XXX
            })
            .ok()?;
            let m = Cw20ExecuteMsg::Send {
                contract: addr.to_string(),
                msg: s,
                amount: amount.u128().into(),
            };
            let b = to_binary(&m).ok()?;
            Some(
                WasmMsg::Execute {
                    contract_addr: addr.to_string(),
                    msg: b,
                    funds: vec![],
                }
                .into(),
            )
        },
        dex::Token::Native { denom } => {
            // native -> token
            let m = ExecuteMsg::Swap {
                offer_asset: Asset {
                    info: AssetInfo::NativeToken {
                        denom: denom.to_string(),
                    },
                    amount: amount.u128().into(),
                },
                belief_price: Some(belief_price),
                max_spread: None, // XXX
                to: None,
            };
            let b = to_binary(&m).ok()?;
            Some(
                WasmMsg::Execute {
                    contract_addr: pool.addr().to_string(),
                    msg: b,
                    funds: coins(amount.u128(), denom),
                }
                .into(),
            )
        },
    }
}

// XXX: this might eventually go into a contract
// XXX: we also need to be aware of our own balances
fn compute_cycle_liq(
    start: &dex::Token,
    left: &Box<dyn dex::Pool>,
    right: &Box<dyn dex::Pool>,
) -> Option<Uint128> {
    let r2 = Decimal256::from_str("0.997").unwrap();

    let [p0_bid, p0_ask] = if left.coins()[0].token == *start {
        left.coins()
    } else {
        let mut ass = left.coins();
        ass.reverse();
        ass
    };

    // XXX: this also matches stable<>stable pairs
    // XXX: filter out stableswap pairs
    let [p1_bid, p1_ask] = if right.coins()[0].token == p0_ask.token {
        right.coins()
    } else {
        let mut ass = right.coins();
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

// XXX: make sure to properly calculate tax
impl Strategy for SinglePair {
    fn execute(&self, pools: &dex::Pools) -> Vec<Vec<terrars::Message>> {
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
            for indices in pairs.iter().permutations(2) {
                if indices.len() != 2 {
                    break;
                }

                let (left, right) = (&pools.0[*(indices[0])], &pools.0[*(indices[1])]);

                if let Some(liq) = compute_cycle_liq(&start, left, right) {
                    log::info!(
                        "got a good trade {} {} to {}",
                        liq,
                        left.addr(),
                        right.addr(),
                    );
                    if let Some((_out, msgs)) = trade_path(&[left, right], &start, liq) {
                        log::info!("{:?}", msgs);
                    }
                }
            }
        }

        // Empty list for now.
        vec![]
    }
}
