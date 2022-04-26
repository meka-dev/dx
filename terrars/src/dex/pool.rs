use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    str::FromStr as _,
};

use astroport::asset::AssetInfo;
use cosmrs::Denom;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{to_binary, Addr, Decimal, Uint128};
use cw20::Cw20ExecuteMsg;
use serde::Deserialize;
use terraswap::pair::Cw20HookMsg;
use tracing::trace;

use crate::{
    base::Coin,
    dex::{Asset, Token, TokenPair},
    wasm::MsgExecuteContract,
    AccountId,
};

pub type TokenIndex = HashMap<Token, Vec<usize>>;
pub type PairIndex = HashMap<TokenPair, Vec<usize>>;

#[derive(Clone, Debug, Deserialize, Eq)]
pub struct Pool {
    pub assets: [Asset; 2],
    pub contract_address: Addr,
    pub liquidity_token: Addr,
    pub total_share: Uint128,
}

impl Hash for Pool {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contract_address.hash(state);
    }
}

impl PartialEq for Pool {
    fn eq(&self, other: &Self) -> bool {
        self.contract_address == other.contract_address
    }
}

impl Pool {
    pub fn ask_token(&self, bid_token: &Token) -> Token {
        if self.assets[0].token == *bid_token {
            self.assets[1].token.clone()
        } else {
            self.assets[0].token.clone()
        }
    }

    pub fn has_token(&self, tok: &Token) -> bool {
        self.assets[0].token == *tok || self.assets[1].token == *tok
    }

    pub fn compute_swap(
        &self,
        bid_token: &Token,
        ask_token: &Token,
        amount: Uint256,
    ) -> Option<(Uint256, Uint256)> {
        let (bid_asset, ask_asset) = match &self.assets {
            [bid, ask] if bid.token == *bid_token && ask.token == *ask_token => (bid, ask),
            [ask, bid] if bid.token == *bid_token && ask.token == *ask_token => (bid, ask),
            _ => return None,
        };

        let ask_reserve: Uint256 = ask_asset.amount.into();
        let bid_reserve: Uint256 = bid_asset.amount.into();

        if amount.is_zero() || bid_reserve.is_zero() || ask_reserve.is_zero() {
            return None;
        }

        let cp = ask_reserve * bid_reserve;
        trace!("cp {:?}", cp);
        let ret: Uint256 = (Decimal256::from_uint256(ask_reserve)
            - Decimal256::from_ratio(cp, bid_reserve + amount))
            * Uint256::one();
        trace!("ret {:?}", ret);
        let spread: Uint256 = (amount * Decimal256::from_ratio(ask_reserve, bid_reserve)) - ret;
        trace!("spread {:?}", spread);
        let fee = ret * Decimal256::permille(3); // XXX: get from chain
        trace!("fee {:?}", fee);
        let out = ret - fee;
        trace!("out {:?}", out);

        Some((out, spread))
    }

    pub fn update(&mut self, bid_token: &Token, in_amount: Uint256, out_amount: Uint256) {
        let (bid_asset_idx, ask_asset_idx) = match &self.assets {
            [bid, _] if bid.token == *bid_token => (0, 1),
            [_, bid] if bid.token == *bid_token => (1, 0),
            _ => return,
        };

        self.assets[bid_asset_idx].amount += Into::<Uint128>::into(in_amount);
        self.assets[ask_asset_idx].amount -= Into::<Uint128>::into(out_amount);

        self.assets[bid_asset_idx].amount_decimal = Asset::to_scaled_decimal256(
            self.assets[bid_asset_idx].amount,
            self.assets[bid_asset_idx].decimals as u32,
        );
        self.assets[ask_asset_idx].amount_decimal = Asset::to_scaled_decimal256(
            self.assets[ask_asset_idx].amount,
            self.assets[ask_asset_idx].decimals as u32,
        );
    }

    pub fn swap(
        &self,
        sender: &AccountId,
        token: &Token,
        amount: Uint256,
        belief_price: Decimal256,
        max_spread: Decimal256,
    ) -> MsgExecuteContract {
        swap(
            self.contract_address.clone(),
            sender,
            token,
            amount,
            belief_price,
            max_spread,
        )
    }

    pub fn swap_arby(&self, token: &Token) -> (Addr, AssetInfo) {
        (self.contract_address.clone(), (*token).clone().into())
    }

    pub fn token_pair(&self) -> TokenPair {
        [self.assets[0].token.clone(), self.assets[1].token.clone()].into()
    }
}

pub struct Pools(pub Vec<Pool>);

impl From<Vec<Pool>> for Pools {
    fn from(input: Vec<Pool>) -> Self {
        Self(input)
    }
}

impl Pools {
    pub fn index_by_token(&self, filter: &Token) -> TokenIndex {
        let mut map = HashMap::new();

        for (index, pool) in self.0.iter().enumerate() {
            if !pool.has_token(filter) {
                continue;
            }

            for asset in &pool.assets {
                let list = map.entry(asset.token.clone()).or_insert_with(Vec::new);
                list.push(index);
            }
        }
        map
    }

    pub fn index_by_pair(&self) -> PairIndex {
        let mut map = PairIndex::new();

        for (index, pool) in self.0.iter().enumerate() {
            let pair = pool.token_pair();
            let list = map.entry(pair).or_insert_with(Vec::new);
            list.push(index);
        }
        map
    }
}

pub fn swap(
    contract_address: Addr,
    sender: &AccountId,
    token: &Token,
    amount: Uint256,
    belief_price: Decimal256,
    max_spread: Decimal256,
) -> MsgExecuteContract {
    use terraswap::{asset, pair};

    let (to, coins, msg) = match token {
        Token::Cw20 { contract_addr } => {
            // token -> native is a send to the token contract
            let swp = Cw20HookMsg::Swap {
                belief_price: Some(belief_price.into()),
                max_spread: Some(max_spread.into()),
                to: None,
            };
            let snd = Cw20ExecuteMsg::Send {
                amount: amount.into(),
                msg: to_binary(&swp).unwrap(),
                contract: contract_address.to_string(),
            };
            (
                contract_addr.clone(),
                vec![],
                serde_json::to_vec(&snd).unwrap(),
            )
        },
        Token::Native { denom } => (
            contract_address,
            vec![Coin {
                amount,
                denom: Denom::from_str(denom).unwrap(),
            }],
            serde_json::to_vec(&pair::ExecuteMsg::Swap {
                offer_asset: asset::Asset {
                    info: asset::AssetInfo::NativeToken {
                        denom: denom.clone(),
                    },
                    amount: amount.into(),
                },
                belief_price: Some(belief_price.into()),
                max_spread: Some(max_spread.into()),
                to: None,
            })
            .unwrap(),
        ),
    };

    MsgExecuteContract {
        coins,
        contract_address: to,
        execute_msg: msg,
        sender: sender.to_string(),
    }
}

// only need this for the actual msg
pub fn belief_price(amount_in: Uint256, amount_out: Uint256) -> Decimal256 {
    Decimal256::from_ratio(amount_in, amount_out)
}

// XXX: astroport has an upper limit on max_spread of 0.5
pub fn max_spread(_amount_out: Uint256, _spread: Uint256) -> Decimal256 {
    // spread / amount_out
    Decimal256::zero()
}

// From terraswap pair contract
// NB(pm): astroport has slightly different logic
pub fn assert_max_spread(
    belief_price: Option<Decimal>,
    max_spread: Option<Decimal>,
    offer_amount: Uint128,
    return_amount: Uint128,
    spread_amount: Uint128,
) -> bool {
    let offer_amount: Uint256 = offer_amount.into();
    let return_amount: Uint256 = return_amount.into();
    let spread_amount: Uint256 = spread_amount.into();

    if let (Some(max_spread), Some(belief_price)) = (max_spread, belief_price) {
        let belief_price: Decimal256 = belief_price.into();
        let max_spread: Decimal256 = max_spread.into();
        // println!("bp {:?} ms {:?}", belief_price, max_spread);

        let expected_return = offer_amount / belief_price;
        let spread_amount = if expected_return > return_amount {
            expected_return - return_amount
        } else {
            Uint256::zero()
        };
        // println!("er {:?} sa {:?}", expected_return, spread_amount);

        // println!("ratio {:?}", Decimal256::from_ratio(spread_amount, expected_return));
        if return_amount < expected_return
            && Decimal256::from_ratio(spread_amount, expected_return) > max_spread
        {
            return false;
        }
    } else if let Some(max_spread) = max_spread {
        let max_spread: Decimal256 = max_spread.into();
        if Decimal256::from_ratio(spread_amount, return_amount + spread_amount) > max_spread {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_bignumber::{Decimal256, Uint256};
    use cosmwasm_std::Addr;
    use terraswap::pair::ExecuteMsg;

    use super::Token;
    use crate::{dex::swap, AccountId};

    #[test]
    fn swap_number_integrity() {
        let addr = Addr::unchecked("foobar");
        let sender = AccountId::from_str("terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302").unwrap();
        let token = Token::Native {
            denom: "umeka".to_owned(),
        };
        let amount = Uint256::from(21249999u128);
        let bp = Decimal256::from_str("1.234").unwrap();
        let ms = Decimal256::from_str("0.005").unwrap();

        let s = swap(addr, &sender, &token, amount, bp, ms);
        assert_eq!(s.coins.first().unwrap().amount, amount);

        let exec: ExecuteMsg = serde_json::from_slice(&s.execute_msg).unwrap();
        if let ExecuteMsg::Swap {
            belief_price,
            max_spread,
            ..
        } = exec
        {
            assert_eq!(belief_price.unwrap().to_string(), bp.to_string());
            assert_eq!(max_spread.unwrap().to_string(), ms.to_string());
        } else {
            panic!("did not produce a swap");
        }
    }
}
