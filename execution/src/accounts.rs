use std::{collections::HashMap, str::FromStr};

use cosmwasm_bignumber::Decimal256;
use terrars::{AccountId, Lcd};

use crate::dex;

pub const PREFIX: &str = "terra";

pub type Balances = HashMap<dex::Token, Decimal256>;

pub async fn fetch(lcd: &Lcd, account_id: &AccountId) -> anyhow::Result<Balances> {
    let res = lcd.bank().balances(account_id.as_ref()).await?;

    let mut balances = HashMap::new();
    for coin in res.result {
        balances.insert(
            dex::Token::Native { denom: coin.denom },
            Decimal256::from_str(&coin.amount.to_string())?,
        );
    }

    Ok(balances)
}
