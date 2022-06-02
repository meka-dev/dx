use std::{collections::HashMap, str::FromStr};

use chainrs::cosmos::base::AccountId;
use common::{err, Report, Result};
use cosmwasm_bignumber::Uint256;
use lcd::Lcd;
use mekapi;
use serde::Serialize;
use strategy;
use tendermint::block::Height;

pub mod osmosis;
pub mod terra;

pub type Balances = HashMap<String, Uint256>;

pub async fn fetch<L>(lcd: L, account_id: AccountId) -> Result<Balances>
where
    L: Lcd,
{
    let coins = lcd.bank_balances(&account_id).await?;

    let mut balances = HashMap::new();
    for coin in coins {
        balances.insert(coin.denom.to_string(), Uint256::from(coin.amount.0));
    }

    Ok(balances)
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Venue {
    Osmosis,
    Terra,
}

impl FromStr for Venue {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Osmosis" => Ok(Self::Osmosis),
            "Terra" => Ok(Self::Terra),
            _ => Err(err!("unknown venue: {}", s)),
        }
    }
}

#[async_trait::async_trait]
pub trait Fetcher<L, M>
where
    L: Lcd,
    M: mekapi::v3::Api,
{
    async fn fetch(
        &self,
        lcd: L,
        mekapi: M,
        account_id: &AccountId,
        height: Height,
    ) -> Result<(Balances, Vec<strategy::Pool>)>;
}
