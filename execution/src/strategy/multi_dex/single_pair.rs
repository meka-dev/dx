#![allow(clippy::borrowed_box)]

use cosmwasm_bignumber::Decimal256;
use itertools::Itertools;
use tendermint::block;

use common::tracing::debug;
use terrars::dex::{self, Token};

use crate::{
    accounts,
    strategy::{
        metrics,
        multi_dex::utils::{compute_cycle_liq, trade_path},
        Strategy, Trade,
    },
};

#[derive(Default)]
pub struct SinglePair {}

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
                        trade_path(sender, [left, right].into_iter(), &start, size)
                    {
                        debug!(
                            "got a good trade {} {} to {}",
                            liq, left.contract_address, right.contract_address,
                        );
                        debug!("size {:#?}", size);
                        debug!("bal {:#?}", bal);
                        // info!("{:?}", messages);
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
