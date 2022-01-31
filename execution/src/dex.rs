use std::collections::{hash_map::Entry, HashMap};

use cosmwasm_bignumber::Decimal256;
use cw20::TokenInfoResponse;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

use terrars::{Lcd, Message};

pub mod astroport;
pub mod terraswap;

pub const DENOM_UST: &str = "uusd";

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
        let amount_decimal = Decimal256::from_uint256(amount.u128())
            / Decimal256::from_uint256(10u128.pow(decimals as u32));

        Self {
            token,
            amount,
            amount_decimal,
            decimals,
        }
    }
}

#[async_trait::async_trait]
pub trait Pool: Send + Sync {
    fn coins(&self) -> [Asset; 2];
    fn swap(&self) -> Message;
    fn token_pair(&self) -> TokenPair;
    async fn update(&mut self, lcd: &Lcd, token_cache: &mut TokenCache) -> anyhow::Result<()>;
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
            &contract_addr.to_string(),
            &serde_json::to_string(&cw20_base::msg::QueryMsg::TokenInfo {})?,
        )
        .await?;

    Ok(res.result)
}
