use std::{collections::HashMap, str::FromStr as _};

use cosmrs::Denom;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{to_binary, Addr, Uint128};
use cw20::Cw20ExecuteMsg;
use serde::Deserialize;
use terraswap::pair::Cw20HookMsg;

use crate::{
    dex::{Asset, Token, TokenPair},
    AccountId, Coin, MsgExecuteContract,
};

pub type TokenIndex = HashMap<Token, Vec<usize>>;
pub type PairIndex = HashMap<TokenPair, Vec<usize>>;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Pool {
    pub assets: [Asset; 2],
    pub contract_address: Addr,
    pub liquidity_token: Addr,
    pub total_share: Uint128,
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
        amount: Decimal256,
    ) -> Option<(Decimal256, Decimal256)> {
        let (bid_asset, ask_asset) = match &self.assets {
            [bid, ask] if bid.token == *bid_token && ask.token == *ask_token => (bid, ask),
            [ask, bid] if bid.token == *bid_token && ask.token == *ask_token => (bid, ask),
            _ => return None,
        };

        let ask_reserve = ask_asset.amount_decimal;
        let bid_reserve = bid_asset.amount_decimal;

        log::debug!(
            "amount {} ask_reserve {} bid_reserve {}",
            amount,
            ask_reserve,
            bid_reserve
        );

        if amount < bid_asset.min_val() {
            return None;
        }

        if bid_reserve == Decimal256::zero() || ask_reserve == Decimal256::zero() {
            return None;
        }

        let cp = ask_reserve * bid_reserve;
        log::debug!("cp {}", cp);
        let ret = ask_reserve - cp / (bid_reserve + amount);
        log::debug!("ret {}", ret);
        let spread = amount * ask_reserve / bid_reserve - ret;
        log::debug!("spread {}", spread);
        let fee = ret * Decimal256::permille(3); // XXX: get from chain
        log::debug!("fee {}", fee);
        let out = ret - fee;
        log::debug!("out {}", out);

        Some((out, spread))
    }
    pub fn swap(
        &self,
        sender: &AccountId,
        token: &Token,
        amount: Decimal256,
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

    pub fn token_pair(&self) -> TokenPair {
        todo!()
    }
}

pub struct Pools(pub Vec<Pool>);

impl From<Vec<Pool>> for Pools {
    fn from(input: Vec<Pool>) -> Self {
        Self(input)
    }
}

impl Pools {
    pub fn index_by_token(&self) -> TokenIndex {
        let mut map = HashMap::new();

        for (index, pool) in self.0.iter().enumerate() {
            for asset in &pool.assets {
                let list = map.entry(asset.token.clone()).or_insert(vec![]);
                list.push(index);
            }
        }
        map
    }

    pub fn index_by_pair(&self) -> PairIndex {
        let mut map = PairIndex::new();

        for (index, pool) in self.0.iter().enumerate() {
            let pair = pool.token_pair();
            let list = map.entry(pair).or_insert(vec![]);
            list.push(index);
        }
        map
    }
}

pub fn swap(
    contract_address: Addr,
    sender: &AccountId,
    token: &Token,
    amount: Decimal256,
    belief_price: Decimal256,
    max_spread: Decimal256,
) -> MsgExecuteContract {
    use terraswap::{asset, pair};

    let normalized_amount = Uint256::from((amount / Decimal256::from_uint256(10u64.pow(12))).0);

    let (to, coins, msg) = match token {
        Token::Cw20 { contract_addr } => {
            // token -> native is a send to the token contract
            let swp = Cw20HookMsg::Swap {
                belief_price: Some(belief_price.into()),
                max_spread: Some(max_spread.into()),
                to: None,
            };
            let snd = Cw20ExecuteMsg::Send {
                amount: normalized_amount.into(),
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
                    amount: normalized_amount.into(),
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

#[cfg(test)]
mod tests {
    use super::Token;
    use crate::{dex::swap, AccountId};
    use std::str::FromStr;
    use terraswap::pair::ExecuteMsg;

    use cosmwasm_bignumber::Decimal256;
    use cosmwasm_std::Addr;

    #[test]
    fn swap_number_integrity() {
        let addr = Addr::unchecked("foobar");
        let sender = AccountId::from_str("terra1eml7g3ll6jkyhtfv2g0gvqnzzpy6kjyd7qr302").unwrap();
        let token = Token::Native {
            denom: "umeka".to_owned(),
        };
        let amount = Decimal256::from_str("21.249999").unwrap();
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
            assert!(false, "did not produce a swap");
        }
    }
}
