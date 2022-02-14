use std::str::FromStr as _;

use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Decimal, Fraction, Uint128};
use serde::{Deserialize, Serialize};
use terraswap::{asset, factory, pair};

use terrars::{AccountId, Lcd, MsgExecuteContract};

use crate::dex::{Asset, Pool, Token, TokenCache, TokenPair};

const LOOP_ADDRESS: &str = "terra16hdjuvghcumu6prg22cdjl96ptuay6r0hc6yns";

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

    fn addr(&self) -> &str {
        &self.contract_addr
    }

    fn swap(
        &self,
        sender: &AccountId,
        token: &Token,
        amount: Decimal256,
        belief_price: Decimal256,
        max_spread: Decimal256,
    ) -> MsgExecuteContract {
        let (coins, msg) = match token {
            Token::Cw20 { contract_addr } => (
                vec![],
                pair::ExecuteMsg::Swap {
                    offer_asset: asset::Asset {
                        info: asset::AssetInfo::Token {
                            contract_addr: contract_addr.clone().to_string(),
                        },
                        amount: Into::<Decimal>::into(amount).numerator().into(), // ????
                    },
                    belief_price: Some(belief_price.into()),
                    max_spread: Some(max_spread.into()),
                    to: None,
                },
            ),
            Token::Native { denom } => (
                // FIXME(xla): Bignumber handling is all over the place, it needs consolidation.
                vec![terrars::Coin {
                    amount: amount / Decimal256::from_uint256(10u128.pow(6)),
                    denom: terrars::Denom::from_str(denom).unwrap(),
                }],
                pair::ExecuteMsg::Swap {
                    offer_asset: asset::Asset {
                        info: asset::AssetInfo::NativeToken {
                            denom: denom.clone(),
                        },
                        amount: Into::<Decimal>::into(amount).numerator().into(), // ????
                    },
                    belief_price: Some(belief_price.into()),
                    max_spread: Some(max_spread.into()),
                    to: None,
                },
            ),
        };

        MsgExecuteContract {
            coins,
            contract: self.contract_addr.clone(),
            execute_msg: serde_json::to_vec(&msg).unwrap(),
            sender: sender.to_string(),
        }
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
        log::debug!("Loading Loop pool info for {}", pair.contract_addr);

        match pool_info(lcd, &pair.contract_addr).await {
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
                    contract_addr: pair.contract_addr.clone(),
                    coins: [left, right],
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
                LOOP_ADDRESS,
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
