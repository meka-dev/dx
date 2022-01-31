use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use terraswap::{asset, factory, pair};

use terrars::Lcd;

use crate::dex::{Asset, Pool, Token, TokenPair};

const TERRASWAP_ADDRESS: &str = "terra1ulgw0td86nvs4wtpsc80thv6xelk76ut7a7apj";

impl From<asset::AssetInfo> for Token {
    fn from(input: asset::AssetInfo) -> Self {
        match input {
            asset::AssetInfo::NativeToken { denom } => Self::Native { denom },
            asset::AssetInfo::Token { contract_addr } => Self::Cw20 {
                contract_addr: Addr::unchecked(contract_addr),
            },
        }
    }
}

impl From<Token> for asset::AssetInfo {
    fn from(input: Token) -> Self {
        match input {
            Token::Native { denom } => Self::NativeToken { denom },
            Token::Cw20 { contract_addr } => Self::Token {
                contract_addr: contract_addr.into(),
            },
        }
    }
}

impl From<asset::Asset> for Asset {
    fn from(input: asset::Asset) -> Self {
        Self {
            amount: input.amount,
            token: input.info.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PairsResult {
    height: String,
    result: factory::PairsResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PairInfoResult {
    height: String,
    result: asset::PairInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PoolResponseResult {
    height: String,
    result: pair::PoolResponse,
}

#[derive(Clone, Debug)]
pub struct Pair {
    pub contract_addr: String,
    pub coins: [Asset; 2],
    pub liquidity_token: String,
    pub total_share: Uint128,
}

#[async_trait::async_trait]
impl Pool for Pair {
    fn coins(&self) -> [Asset; 2] {
        let mut coins = self.coins.clone();
        coins.sort();
        coins
    }

    fn swap(&self) -> terrars::Message {
        todo!()
    }

    fn token_pair(&self) -> TokenPair {
        let mut coins = self.coins.clone();
        coins.sort();
        [coins[0].token.clone(), coins[1].token.clone()].into()
    }

    async fn update(&mut self, lcd: &Lcd) -> anyhow::Result<()> {
        let pool = pool_info(lcd, &self.contract_addr).await?;
        self.coins = [pool.assets[0].clone().into(), pool.assets[1].clone().into()];

        Ok(())
    }
}

pub async fn liquid_pairs(lcd: &Lcd, filter: Option<Token>) -> anyhow::Result<Vec<Box<dyn Pool>>> {
    let ps = pairs(lcd).await?.into_iter().filter(|pair| {
        if let Some(coin) = filter.clone() {
            pair.asset_infos[0].equal(&coin.clone().into())
                || pair.asset_infos[1].equal(&coin.into())
        } else {
            true
        }
    });

    let mut pairs: Vec<Box<dyn Pool>> = vec![];
    for pair in ps {
        log::debug!("Loading Terraswap pool info for {}", pair.contract_addr);

        match pool_info(lcd, &pair.contract_addr).await {
            Ok(pool) => {
                if pool.total_share == Uint128::zero()
                    || pool.assets[0].amount == Uint128::zero()
                    || pool.assets[1].amount == Uint128::zero()
                {
                    continue;
                }

                pairs.push(Box::new(Pair {
                    contract_addr: pair.contract_addr.clone(),
                    coins: [pool.assets[0].clone().into(), pool.assets[1].clone().into()],
                    liquidity_token: pair.liquidity_token.clone(),
                    total_share: pool.total_share,
                }));
            },
            Err(err) => log::error!(
                "Error fetching pool info for {}: {}",
                pair.contract_addr,
                err
            ),
        }
    }

    Ok(pairs)
}

async fn pairs(lcd: &Lcd) -> anyhow::Result<Vec<asset::PairInfo>> {
    let mut pairs: Vec<asset::PairInfo> = vec![];
    let mut start_after: Option<[asset::AssetInfo; 2]> = None;
    let limit = 30;

    loop {
        let mut res = lcd
            .wasm()
            .query::<PairsResult>(
                TERRASWAP_ADDRESS,
                &serde_json::to_string(&factory::QueryMsg::Pairs {
                    start_after,
                    limit: Some(30),
                })?,
            )
            .await?;

        start_after = res.result.pairs.last().map(|pair| pair.asset_infos.clone());
        let size = res.result.pairs.len();

        pairs.append(&mut res.result.pairs);

        if size < limit {
            break;
        }
    }

    Ok(pairs)
}

async fn pool_info(lcd: &Lcd, contract_addr: &str) -> anyhow::Result<pair::PoolResponse> {
    let res = lcd
        .wasm()
        .query::<PoolResponseResult>(
            contract_addr,
            &serde_json::to_string(&pair::QueryMsg::Pool {})?,
        )
        .await?;

    Ok(res.result)
}
