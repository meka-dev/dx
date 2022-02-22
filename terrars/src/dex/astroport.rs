use std::str::FromStr as _;

use astroport::{asset, factory, pair};
use cosmrs::Denom;
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{Addr, Decimal, Fraction, Uint128};

use crate::{
    base::Coin,
    dex::{self, Pool, Token, TokenCache, TokenPair},
    lcd::Lcd,
    AccountId, MsgExecuteContract,
};

pub use astroport::{
    asset::{Asset, AssetInfo},
    pair::PoolResponse,
};

pub const ADDRESS: &str = "terra1fnywlw4edny3vw44x04xd67uzkdqluymgreu7g";

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

#[derive(Clone, Debug)]
pub struct Pair {
    pub contract_address: Addr,
    pub coins: [dex::Asset; 2],
    pub liquidity_token: String,
    pub total_share: Uint128,
}

#[async_trait::async_trait]
impl<L> Pool<L> for Pair
where
    L: Lcd,
{
    fn coins(&self) -> [dex::Asset; 2] {
        let mut coins = self.coins.clone();
        coins.sort();
        coins
    }

    fn addr(&self) -> Addr {
        self.contract_address.clone()
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
                            contract_addr: contract_addr.clone(),
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
                vec![Coin {
                    amount,
                    denom: Denom::from_str(denom).unwrap(),
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
            contract_address: self.contract_address.clone(),
            execute_msg: serde_json::to_vec(&msg).unwrap(),
            sender: sender.to_string(),
        }
    }

    fn token_pair(&self) -> TokenPair {
        let mut coins = self.coins.clone();
        coins.sort();
        [coins[0].token.clone(), coins[1].token.clone()].into()
    }

    async fn update(&mut self, lcd: &L, token_cache: &mut TokenCache) -> anyhow::Result<()> {
        let pool = pool_info(lcd, &self.contract_address).await?;
        let left = into_asset(lcd, token_cache, pool.assets[0].clone()).await?;
        let right = into_asset(lcd, token_cache, pool.assets[1].clone()).await?;
        self.coins = [left, right];

        Ok(())
    }
}

async fn into_asset<L>(
    lcd: &L,
    token_cache: &mut TokenCache,
    input: asset::Asset,
) -> anyhow::Result<dex::Asset>
where
    L: Lcd,
{
    let token = Token::from(input.info);

    match token {
        Token::Cw20 { ref contract_addr } => {
            let info = token_cache.get(lcd, contract_addr).await?;
            Ok(dex::Asset::from((token, input.amount, info.decimals)))
        },
        Token::Native { .. } => Ok(dex::Asset::from((token, input.amount, 6))),
    }
}

pub async fn liquid_pairs<L>(
    lcd: &L,
    token_cache: &mut TokenCache,
    filter: Option<Token>,
) -> anyhow::Result<Vec<Box<dyn Pool<L>>>>
where
    L: Lcd,
{
    let ps = pairs(lcd).await?.into_iter().filter(|pair| {
        if let Some(coin) = filter.clone() {
            pair.asset_infos[0].equal(&coin.clone().into())
                || pair.asset_infos[1].equal(&coin.into())
        } else {
            true
        }
    });

    let mut pairs: Vec<Box<dyn Pool<L>>> = vec![];
    for pair in ps {
        log::debug!("Loading Astroport pool info for {}", pair.contract_addr);

        let contract_address = Addr::unchecked(pair.contract_addr.clone());
        match pool_info(lcd, &contract_address).await {
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
                    contract_address,
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

async fn pairs<L>(lcd: &L) -> anyhow::Result<Vec<asset::PairInfo>>
where
    L: Lcd,
{
    let mut pairs: Vec<asset::PairInfo> = vec![];
    let mut start_after: Option<[asset::AssetInfo; 2]> = None;
    let limit = 30;

    loop {
        let mut res = lcd
            .wasm_contract_store::<factory::QueryMsg, factory::PairsResponse>(
                &Addr::unchecked(ADDRESS),
                &factory::QueryMsg::Pairs {
                    start_after,
                    limit: Some(30),
                },
            )
            .await?;

        start_after = res.pairs.last().map(|pair| pair.asset_infos.clone());
        let size = res.pairs.len();

        pairs.append(&mut res.pairs);

        if size < limit {
            break;
        }
    }

    Ok(pairs)
}

async fn pool_info<L>(lcd: &L, contract_addr: &Addr) -> anyhow::Result<pair::PoolResponse>
where
    L: Lcd,
{
    let res = lcd
        .wasm_contract_store::<pair::QueryMsg, pair::PoolResponse>(
            contract_addr,
            &pair::QueryMsg::Pool {},
        )
        .await?;

    Ok(res)
}
