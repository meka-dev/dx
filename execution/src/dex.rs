use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

use terrars::{Lcd, Message};

pub mod astroport;
pub mod terraswap;

const DENOM_USD: &str = "uusd";

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct AssetInfoPair(pub [AssetInfo; 2]);

impl From<[AssetInfo; 2]> for AssetInfoPair {
    fn from(input: [AssetInfo; 2]) -> Self {
        Self(input)
    }
}

pub type PairIndex = HashMap<AssetInfoPair, Vec<usize>>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AssetInfo {
    NativeToken { denom: String },
    Token { contract_addr: Addr },
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Asset {
    pub amount: Uint128,
    pub info: AssetInfo,
}

impl Asset {
    pub fn is_native(&self) -> bool {
        matches!(self.info, AssetInfo::NativeToken { .. })
    }

    pub fn is_ust(&self) -> bool {
        matches!(self.info, AssetInfo::NativeToken { ref denom } if denom == &DENOM_USD.to_string())
    }
}

#[async_trait::async_trait]
pub trait Pool: Send + Sync {
    fn info_pair(&self) -> AssetInfoPair;
    fn coins(&self) -> [Asset; 2];
    fn swap(&self) -> Message;
    async fn update(&mut self, lcd: &Lcd) -> anyhow::Result<()>;
}

pub struct Pools(pub Vec<Box<dyn Pool>>);

impl From<Vec<Box<dyn Pool>>> for Pools {
    fn from(input: Vec<Box<dyn Pool>>) -> Self {
        Self(input)
    }
}

impl Pools {
    pub fn index_by_asset(&self) -> HashMap<AssetInfo, Vec<&Box<dyn Pool>>> {
        let mut map = HashMap::new();

        for (index, pool) in self.0.iter().enumerate() {
            let c = pool.coins();

            let info = if c[0].is_native() { c[1].info.clone() } else { c[0].info.clone() };

            let list = map.entry(info).or_insert(vec![]);
            list.push(pool);
        }
        map
    }

    #[allow(clippy::or_fun_call)]
    pub fn index_by_pair(&self) -> PairIndex {
        let mut map = PairIndex::new();

        for (index, pool) in self.0.iter().enumerate() {
            let pair = pool.info_pair();
            let list = map.entry(pair).or_insert(vec![]);
            list.push(index);
        }
        map
    }
}

pub type Tokens = HashMap<Addr, cw20::TokenInfoResponse>;

pub async fn get_token_infos(lcd: &Lcd, pools: &Pools) -> anyhow::Result<Tokens> {
    let mut tokens = HashMap::new();

    for pool in &pools.0 {
        let pair = pool.info_pair();

        if let AssetInfo::Token { ref contract_addr } = pair.0[0] {
            if !tokens.contains_key(contract_addr) {
                let info = token_info(lcd, contract_addr).await?;
                tokens.insert(contract_addr.clone(), info);
            }
        }

        if let AssetInfo::Token { ref contract_addr } = pair.0[1] {
            if !tokens.contains_key(contract_addr) {
                let info = token_info(lcd, contract_addr).await?;
                tokens.insert(contract_addr.clone(), info);
            }
        }
    }

    Ok(tokens)
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
