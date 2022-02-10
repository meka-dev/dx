use std::collections::{hash_map::Entry, HashMap};

use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw20::TokenInfoResponse;
use serde::{Deserialize, Serialize};

use terrars::{AccountId, Lcd, MsgExecuteContract};

pub mod astroport;
pub mod loopmarkets;
pub mod terraswap;

pub const DENOM_UST: &str = "uusd";
pub const DENOM_LUNA: &str = "uluna";

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TokenPair(pub [Token; 2]);

impl From<[Token; 2]> for TokenPair {
    fn from(input: [Token; 2]) -> Self {
        Self(input)
    }
}

pub type TokenIndex = HashMap<Token, Vec<usize>>;
pub type PairIndex = HashMap<TokenPair, Vec<usize>>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Token {
    Cw20 { contract_addr: Addr },
    Native { denom: String },
}

impl Token {
    pub fn is_native(&self) -> bool {
        matches!(self, Token::Native { .. })
    }

    pub fn luna() -> Self {
        Self::Native {
            denom: DENOM_LUNA.to_string(),
        }
    }

    pub fn is_luna(&self) -> bool {
        *self == Self::luna()
    }

    pub fn ust() -> Self {
        Self::Native {
            denom: DENOM_UST.to_string(),
        }
    }

    pub fn is_ust(&self) -> bool {
        *self == Self::ust()
    }
}

#[derive(Clone, Debug)]
pub struct Asset {
    pub token: Token,
    pub amount: Uint128,
    pub amount_decimal: Decimal256,
    pub decimals: u8,
}

impl Eq for Asset {}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

impl Ord for Asset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.token.cmp(&other.token)
    }
}

impl PartialOrd for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(Token, Uint128, u8)> for Asset {
    fn from((token, amount, decimals): (Token, Uint128, u8)) -> Self {
        let amount_decimal = to_scaled_decimal256(amount.u128(), decimals as u32);

        Self {
            token,
            amount,
            amount_decimal,
            decimals,
        }
    }
}

fn to_scaled_decimal256<T: Into<Uint256>>(n: T, decimals: u32) -> Decimal256 {
    Decimal256::from_uint256(n) / Decimal256::from_uint256(10u128.pow(decimals))
}

#[async_trait::async_trait]
pub trait Pool: Send + Sync {
    fn coins(&self) -> [Asset; 2];
    fn addr(&self) -> &str;
    fn swap(
        &self,
        sender: &AccountId,
        token: &Token,
        amount: Uint128,
        belief_price: Decimal,
    ) -> MsgExecuteContract;
    fn token_pair(&self) -> TokenPair;
    async fn update(&mut self, lcd: &Lcd, token_cache: &mut TokenCache) -> anyhow::Result<()>;

    fn compute_swap(
        &self,
        bid_token: &Token,
        ask_token: &Token,
        amount: Uint128,
    ) -> Option<(Uint128, Uint128)> {
        if amount == Uint128::zero() {
            return None;
        }

        let (bid_reserve, ask_reserve, amt) = match self.coins() {
            [bid, ask] if bid.token == *bid_token && ask.token == *ask_token => {
                let amt = to_scaled_decimal256(amount, bid.decimals as u32);
                (bid.amount_decimal, ask.amount_decimal, amt)
            },
            [ask, bid] if bid.token == *bid_token && ask.token == *ask_token => {
                let amt = to_scaled_decimal256(amount, bid.decimals as u32);
                (bid.amount_decimal, ask.amount_decimal, amt)
            },
            _ => return None,
        };
        log::debug!(
            "amt {} ask_reserve {} bid_reserve {}",
            amt,
            ask_reserve,
            bid_reserve
        );

        if bid_reserve == Decimal256::zero()
            || ask_reserve == Decimal256::zero()
            || amt == Decimal256::zero()
        {
            return None;
        }

        let cp = ask_reserve * bid_reserve;
        log::debug!("cp {}", cp);
        let ret = ask_reserve - cp / (bid_reserve + amt);
        log::debug!("ret {}", ret);
        let spread = amt * ask_reserve / bid_reserve - ret;
        log::debug!("spread {}", spread);
        let fee = ret * Decimal256::permille(3); // XXX: get from chain
        log::debug!("fee {}", fee);
        let out = ret - fee;
        log::debug!("out {}", out);

        let spread_scaled: Uint256 = if spread != Decimal256::zero() {
            (spread / Decimal256::from_uint256(10u128.pow(12))).0.into()
        } else {
            Uint256::zero()
        };
        let out_scaled: Uint256 = if out != Decimal256::zero() {
            (out / Decimal256::from_uint256(10u128.pow(12))).0.into()
        } else {
            Uint256::zero()
        };

        Some((out_scaled.into(), spread_scaled.into()))
    }

    fn ask_token(&self, bid_token: &Token) -> Token {
        if self.coins()[0].token == *bid_token {
            self.coins()[1].token.clone()
        } else {
            self.coins()[0].token.clone()
        }
    }
}

pub struct Pools(pub Vec<Box<dyn Pool>>);

impl From<Vec<Box<dyn Pool>>> for Pools {
    fn from(input: Vec<Box<dyn Pool>>) -> Self {
        Self(input)
    }
}

#[allow(clippy::or_fun_call)]
impl Pools {
    pub fn index_by_token(&self) -> TokenIndex {
        let mut map = HashMap::new();

        for (index, pool) in self.0.iter().enumerate() {
            for coin in pool.coins() {
                let list = map.entry(coin.token).or_insert(vec![]);
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

#[derive(Default)]
pub struct TokenCache(pub HashMap<Addr, cw20::TokenInfoResponse>);

impl TokenCache {
    pub async fn get(
        &mut self,
        lcd: &Lcd,
        contract_addr: &Addr,
    ) -> anyhow::Result<TokenInfoResponse> {
        match self.0.entry(contract_addr.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let info = token_info(lcd, contract_addr).await?;
                entry.insert(info.clone());
                Ok(info)
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct TokenInfoResult {
    height: String,
    result: cw20::TokenInfoResponse,
}

async fn token_info(lcd: &Lcd, contract_addr: &Addr) -> anyhow::Result<cw20::TokenInfoResponse> {
    let res = lcd
        .wasm()
        .query::<TokenInfoResult>(
            contract_addr.as_ref(),
            &serde_json::to_string(&cw20_base::msg::QueryMsg::TokenInfo {})?,
        )
        .await?;

    Ok(res.result)
}
