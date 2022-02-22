use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr as _,
};

use cosmrs::Denom;
use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::{Addr, Uint128};
use cw20::TokenInfoResponse;

use crate::{lcd::Lcd, AccountId, Coin, MsgExecuteContract};

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

impl Asset {
    fn min_val(&self) -> Decimal256 {
        Decimal256::from_ratio(
            Uint256::one(),
            Into::<Uint256>::into(10u128.pow(self.decimals as u32)),
        )
    }
}

pub fn to_scaled_decimal256<T: Into<Uint256>>(n: T, decimals: u32) -> Decimal256 {
    Decimal256::from_uint256(n) / Decimal256::from_uint256(10u128.pow(decimals))
}

#[async_trait::async_trait]
pub trait Pool<L: Lcd>: Send + Sync {
    fn coins(&self) -> [Asset; 2];
    fn addr(&self) -> Addr;
    fn swap(
        &self,
        sender: &AccountId,
        token: &Token,
        amount: Decimal256,
        belief_price: Decimal256,
        max_spread: Decimal256,
    ) -> MsgExecuteContract {
        swap(self.addr(), sender, token, amount, belief_price, max_spread)
    }

    fn token_pair(&self) -> TokenPair;
    async fn update(&mut self, lcd: &L, token_cache: &mut TokenCache) -> anyhow::Result<()>;

    fn compute_swap(
        &self,
        bid_token: &Token,
        ask_token: &Token,
        amount: Decimal256,
    ) -> Option<(Decimal256, Decimal256)> {
        let (bid_asset, ask_asset) = match self.coins() {
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

    fn ask_token(&self, bid_token: &Token) -> Token {
        if self.coins()[0].token == *bid_token {
            self.coins()[1].token.clone()
        } else {
            self.coins()[0].token.clone()
        }
    }
}

pub struct Pools<L>(pub Vec<Box<dyn Pool<L>>>);

impl<L> From<Vec<Box<dyn Pool<L>>>> for Pools<L> {
    fn from(input: Vec<Box<dyn Pool<L>>>) -> Self {
        Self(input)
    }
}

#[allow(clippy::or_fun_call)]
impl<L> Pools<L>
where
    L: Lcd,
{
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
    pub async fn get<L>(
        &mut self,
        lcd: &L,
        contract_addr: &Addr,
    ) -> anyhow::Result<TokenInfoResponse>
    where
        L: Lcd,
    {
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

async fn token_info<L>(lcd: &L, contract_addr: &Addr) -> anyhow::Result<cw20::TokenInfoResponse>
where
    L: Lcd,
{
    let res = lcd
        .wasm_contract_store::<cw20_base::msg::QueryMsg, cw20::TokenInfoResponse>(
            contract_addr,
            &cw20_base::msg::QueryMsg::TokenInfo {},
        )
        .await?;

    Ok(res)
}

pub fn swap(
    contract_address: Addr,
    sender: &AccountId,
    token: &Token,
    amount: Decimal256,
    belief_price: Decimal256,
    max_spread: Decimal256,
) -> MsgExecuteContract {
    use ::terraswap::{asset, pair};

    let normalized_amount = Uint256::from((amount / Decimal256::from_uint256(10u64.pow(12))).0);
    let normalized_belief_price = belief_price * Decimal256::from_uint256(10u64.pow(6));
    let normalized_max_spread = max_spread * Decimal256::from_uint256(10u64.pow(6));

    let (coins, msg) = match token {
        Token::Cw20 { contract_addr } => (
            vec![],
            pair::ExecuteMsg::Swap {
                offer_asset: asset::Asset {
                    info: asset::AssetInfo::Token {
                        contract_addr: contract_addr.clone().to_string(),
                    },
                    amount: normalized_amount.into(), // ????
                },
                belief_price: Some(normalized_belief_price.into()),
                max_spread: Some(normalized_max_spread.into()),
                to: None,
            },
        ),
        Token::Native { denom } => (
            vec![Coin {
                amount,
                denom: Denom::from_str(denom).unwrap(),
            }],
            pair::ExecuteMsg::Swap {
                offer_asset: asset::Asset {
                    info: asset::AssetInfo::NativeToken {
                        denom: denom.clone(),
                    },
                    amount: normalized_amount.into(),
                },
                belief_price: Some(normalized_belief_price.into()),
                max_spread: None,
                to: None,
            },
        ),
    };

    MsgExecuteContract {
        coins,
        contract_address,
        execute_msg: serde_json::to_vec(&msg).unwrap(),
        sender: sender.to_string(),
    }
}
