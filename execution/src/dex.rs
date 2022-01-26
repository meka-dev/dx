use std::collections::HashMap;

use cosmwasm_std::Uint128;

use terrars::{Lcd, Message};

pub mod astroport;
pub mod terraswap;

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
    Token { contract_addr: String },
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Asset {
    pub amount: Uint128,
    pub info: AssetInfo,
}

impl Asset {
    pub fn is_native(&self) -> bool {
        match self.info {
            AssetInfo::NativeToken { .. } => true,
            _ => false,
        }
    }

    pub fn is_ust(&self) -> bool {
        match self.info {
            AssetInfo::NativeToken { ref denom } if denom == &"uusd".to_string() => true,
            _ => false,
        }
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
    fn index_by_asset(&self) -> HashMap<AssetInfo, Vec<Box<dyn Pool>>> {
        todo!()
    }

    fn index_by_pair(&self) -> PairIndex {
        let mut map = PairIndex::new();

        for (index, pool) in self.0.iter().enumerate() {
            let pair = pool.info_pair();
            let list = map.entry(pair).or_insert(vec![index]);
            list.push(index);
        }
        map
    }
}
