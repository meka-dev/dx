use std::collections::HashMap;

use chainrs::cosmos::base::AccountId;
use common::Result;
use cosmwasm_bignumber::Uint256;
use lcd::Lcd;
use strategy::{Balances, Token};

pub const PREFIX: &str = "terra";

pub async fn fetch<L>(lcd: L, account_id: AccountId) -> Result<Balances>
where
    L: Lcd,
{
    let coins = lcd.bank_balances(&account_id).await?;

    let mut balances = HashMap::new();
    for coin in coins {
        balances.insert(
            Token {
                id: coin.denom.to_string(),
                decimals: 6,
                native: true,
            },
            Uint256::from(coin.amount.0),
        );
    }

    Ok(balances)
}
