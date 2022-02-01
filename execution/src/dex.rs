use std::{
    collections::{hash_map::Entry, HashMap},
};

use cosmwasm_bignumber::{Decimal256, Uint256};
use cw20::TokenInfoResponse;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

use terrars::{Lcd, Message};

pub mod astroport;
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

    pub fn is_ust(&self) -> bool {
        matches!(self, Token::Native { ref denom, .. } if denom == &DENOM_UST.to_string())
    }

    pub fn is_luna(&self) -> bool {
        matches!(self, Token::Native { ref denom, .. } if denom == &DENOM_LUNA.to_string())
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
    fn swap(&self) -> Message;
    fn token_pair(&self) -> TokenPair;
    async fn update(&mut self, lcd: &Lcd, token_cache: &mut TokenCache) -> anyhow::Result<()>;

    fn compute_swap(
        &self,
        bid_token: &Token,
        ask_token: &Token,
        amount: Uint128,
    ) -> Option<(Uint128, Uint128)> {
        let (ask_reserve, bid_reserve, amt) = match self.coins() {
            [bid, ask] if bid.token == *bid_token && ask.token == *ask_token => {
                let a = to_scaled_decimal256(amount.u128(), bid.decimals as u32);
                (ask.amount_decimal, bid.amount_decimal, a)
            },
            [ask, bid] if bid.token == *bid_token && ask.token == *ask_token => {
                let a = to_scaled_decimal256(amount.u128(), bid.decimals as u32);
                (bid.amount_decimal, ask.amount_decimal, a)
            },
            _ => return None,
        };

        let cp = ask_reserve * bid_reserve;
        let ret = ask_reserve - cp / (bid_reserve + amt);
        let spread: Uint256 = (amt * ask_reserve / bid_reserve - ret).0.into();
        let fee = ret * Decimal256::permille(3); // XXX: get from chain
        let out: Uint256 = (ret - fee).0.into();

        return Some((out.into(), spread.into()));
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

impl Clone for Box<dyn Pool> {
    fn clone(&self) -> Self {
        todo!()
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
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

    // Iterate through a bunch of pools and properly arrang them s.t.
    // asset0 is always bid and asset1 is the ask.
    // XXX: do we actually want to implement this here?
    pub fn trade_path(&self, start: &Token, amount: Uint128) -> Option<Uint128> {
        let mut cb = amount;
        let mut spread = Uint128::zero();
        let mut bid_token = start.clone();

        for p in self.0.iter() {
            let ask_token = p.ask_token(&bid_token);
            if let Some((r, s)) = p.compute_swap(&bid_token, &ask_token, cb) {
                spread = s;
                log::info!(
                    "swap {} {:?} for {} {:?} with a spread of {}",
                    cb,
                    bid_token,
                    r,
                    ask_token,
                    spread
                );
                cb = r;
                bid_token = ask_token;
            }
        }

        return None;
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
