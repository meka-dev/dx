use serde::{Deserialize, Serialize};

use astroport::asset;
use astroport::factory;
use astroport::pair;
use cosmwasm_std::Addr;
use cosmwasm_std::Uint128;

use terrars::Lcd;

use crate::dex::{Asset, Pool, Token, TokenCache, TokenPair};

const ASTROPORT_ADDRESS: &str = "terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g";

impl From<asset::AssetInfo> for Token {
    fn from(input: asset::AssetInfo) -> Self {
        match input {
            asset::AssetInfo::NativeToken { denom } => Self::Native { denom },
            asset::AssetInfo::Token { contract_addr } => Self::Cw20 { contract_addr },
        }
    }
}

impl From<Token> for asset::AssetInfo {
    fn from(input: Token) -> Self {
        match input {
            Token::Native { denom } => Self::NativeToken { denom },
            Token::Cw20 { contract_addr } => Self::Token {
                contract_addr: Addr::unchecked(contract_addr),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PoolResponseResult {
    height: String,
    result: pair::PoolResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct PairsResult {
    height: String,
    result: factory::PairsResponse,
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

    fn addr(&self) -> &str {
        &self.contract_addr
    }

    fn swap(&self) -> terrars::Message {
        todo!()
    }

    fn token_pair(&self) -> TokenPair {
        let mut coins = self.coins.clone();
        coins.sort();
        [coins[0].token.clone(), coins[1].token.clone()].into()
    }

    async fn update(&mut self, lcd: &Lcd, token_cache: &mut TokenCache) -> anyhow::Result<()> {
        let pool = pool_info(lcd, &self.contract_addr).await?;
        let left = into_asset(lcd, token_cache, pool.assets[0].clone()).await?;
        let right = into_asset(lcd, token_cache, pool.assets[1].clone()).await?;
        self.coins = [left, right];

        Ok(())
    }
}

async fn into_asset(
    lcd: &Lcd,
    token_cache: &mut TokenCache,
    input: asset::Asset,
) -> anyhow::Result<Asset> {
    let token = Token::from(input.info);

    match token {
        Token::Cw20 { ref contract_addr } => {
            let info = token_cache.get(lcd, contract_addr).await?;
            Ok(Asset::from((token, input.amount, info.decimals)))
        },
        Token::Native { .. } => Ok(Asset::from((token, input.amount, 6))),
    }
}

pub async fn liquid_pairs(
    lcd: &Lcd,
    token_cache: &mut TokenCache,
    filter: Option<Token>,
) -> anyhow::Result<Vec<Box<dyn Pool>>> {
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
        log::debug!("Loading Astroport pool info for {}", pair.contract_addr);

        match pool_info(lcd, pair.contract_addr.as_str()).await {
            Ok(pool) => {
                if pool.total_share == Uint128::zero()
                    || pool.assets[0].amount == Uint128::zero()
                    || pool.assets[1].amount == Uint128::zero()
                {
                    continue;
                }

                let left = into_asset(lcd, token_cache, pool.assets[0].clone()).await?;
                let right = into_asset(lcd, token_cache, pool.assets[1].clone()).await?;

                pairs.push(Box::new(Pair {
                    contract_addr: pair.contract_addr.to_string(),
                    coins: [left, right],
                    liquidity_token: pair.liquidity_token.to_string(),
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
                ASTROPORT_ADDRESS,
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
