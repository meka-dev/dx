use std::collections::HashMap;

use chainrs::cosmos::{auth::BaseAccount, base::AccountId};
use common::{err, Result};
use cosmwasm_bignumber::Uint256;
use lcd::Lcd;
use strategy::{Balances, Token};

pub const PREFIX: &str = "osmo";

pub async fn balances<L>(lcd: L, account_id: AccountId) -> Result<Balances>
where
    L: Lcd,
{
    let coins = lcd.bank_balances(&account_id).await?;

    let mut balances = HashMap::new();
    for coin in coins {
        balances.insert(
            Token {
                id: coin.denom.to_string(),
                decimals: 6, // XXX
                native: true,
            },
            Uint256::from(coin.amount.0),
        );
    }

    Ok(balances)
}

pub async fn base<L>(lcd: L, account_id: AccountId) -> Result<BaseAccount>
where
    L: Lcd,
{
    let base = lcd
        .auth_account(&account_id)
        .await?
        .ok_or(err!("account not found"))?;

    Ok(base)
}
