#![allow(clippy::borrowed_box)]

use std::str::FromStr as _;

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::Decimal256 as StdDecimal256;
use itertools::Itertools;
use tendermint::block;
use terrars::{AccountId, MsgExecuteContract};

use crate::{
    accounts, dex,
    strategy::{metrics, Strategy, Trade},
};

#[derive(Default)]
pub struct SinglePair {}

// Iterate through a bunch of pools and properly arrange them s.t.
// asset0 is always bid and asset1 is the ask.
// XXX: do we actually want to implement this here?
fn trade_path(
    sender: &AccountId,
    pools: &[&Box<dyn dex::Pool>; 2],
    start: &dex::Token,
    amount: Decimal256,
) -> Option<(Decimal256, Vec<MsgExecuteContract>)> {
    let mut msgs = vec![];
    let mut cb = amount;
    let mut bid_token = start.clone();

    for p in pools {
        let ask_token = p.ask_token(&bid_token);
        if let Some((r, s)) = p.compute_swap(&bid_token, &ask_token, cb) {
            if r == Decimal256::zero() || s == Decimal256::zero() {
                return None;
            }

            log::info!(
                "swap {} {:?} for {} {:?} with a spread of {} on {}",
                cb,
                bid_token,
                r,
                ask_token,
                s,
                p.addr(),
            );
            msgs.push(p.swap(sender, &bid_token, r, belief_price(cb, r), s));

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
fn belief_price(amount_in: Decimal256, amount_out: Decimal256) -> Decimal256 {
    amount_in / amount_out
}

// XXX: this might eventually go into a contract
// XXX: we also need to be aware of our own balances
fn compute_cycle_liq(
    start: &dex::Token,
    left: &Box<dyn dex::Pool>,
    right: &Box<dyn dex::Pool>,
) -> Option<Decimal256> {
    let r2 = Decimal256::from_str("0.997").unwrap();

    let [p0_bid, p0_ask] = if left.coins()[0].token == *start {
        left.coins()
    } else {
        let mut ass = left.coins();
        ass.reverse();
        ass
    };

    let [p1_bid, p1_ask] = if right.coins()[0].token == p0_ask.token {
        right.coins()
    } else {
        let mut ass = right.coins();
        ass.reverse();
        ass
    };

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

    let op_std = StdDecimal256::from_str(&(r2 * a * a_p).to_string())
        .unwrap()
        .sqrt();
    let op = Decimal256::from_str(&op_std.to_string()).unwrap();

    log::debug!("op {:}", op);
    log::debug!("a {:}", a);
    log::debug!("a_p {:}", a_p);
    if op < a {
        return None;
    }

    let delta = op - a;
    let t: Decimal256 = Decimal256::from_ratio(
        Uint256::one(),
        Into::<Uint256>::into(10u128.pow(p0_bid.decimals as u32)),
    );
    log::debug!("delta {:}", delta);
    if delta < t {
        log::debug!("compute_swap below threshold");
        // If we are getting out less than the precision of the token then we just
        // return None
        return None;
    }
    Some(delta)
}

impl metrics::Labeled for SinglePair {
    const MODULE: metrics::Module = metrics::Module::MultiDex;
    const NAME: metrics::Name = metrics::Name::SinglePair;
}

// XXX: make sure to properly calculate tax
#[async_trait::async_trait]
impl Strategy for SinglePair {
    async fn execute(
        &self,
        height: block::Height,
        sender: &terrars::AccountId,
        funds: &accounts::Balances,
        pools: &dex::Pools,
    ) -> Vec<Trade> {
        let mut trades = vec![];

        // crazy arbs go here
        let start = dex::Token::ust();

        log::info!("#{}", height);
        for (token, pairs) in pools.index_by_token() {
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
                log::debug!(
                    "compute liq for path left {:} right {:}",
                    left.addr(),
                    right.addr()
                );

                if let Some(liq) = compute_cycle_liq(&start, left, right) {
                    log::info!(
                        "got a good trade {} {} to {}",
                        liq,
                        left.addr(),
                        right.addr(),
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
                    if let Some((expected_profit, messages)) =
                        trade_path(sender, &[left, right], &start, size)
                    {
                        log::info!("{:?}", messages);
                        trades.push(Trade {
                            size,
                            expected_profit,
                            messages,
                        });
                    } else {
                        log::info!(
                            "but could not seize the opportunity with my balance {:}",
                            bal
                        );
                    }
                }
            }
        }

        trades
    }
}
