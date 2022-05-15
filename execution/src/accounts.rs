use std::collections::HashMap;

use common::Result;
use cosmwasm_bignumber::Uint256;
use terrars::{dex, lcd::Lcd, AccountId};

pub const PREFIX: &str = "terra";

pub type Balances = HashMap<dex::Token, Uint256>;

pub async fn fetch<L>(lcd: L, account_id: AccountId) -> Result<Balances>
where
    L: Lcd,
{
    let coins = lcd.bank_balances(&account_id).await?;

    let mut balances = HashMap::new();
    for coin in coins {
        balances.insert(
            dex::Token::Native {
                denom: coin.denom.to_string(),
            },
            coin.amount,
        );
    }

    Ok(balances)
}
