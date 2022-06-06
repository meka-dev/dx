use std::sync::{Arc, RwLock};

use address_book::AddressBook;
use common::{
    metrics::Registry,
    spawn::flatten,
    tracing::{debug, error},
    Result,
};
use cosmwasm_bignumber::Uint256;
use lcd::Lcd;
use prometheus_client::{
    encoding::text::Encode,
    metrics::{family::Family, gauge::Gauge},
};
use strategy::Token;
use tendermint::block;
use tokio::sync::watch;

use crate::accounts;

pub async fn run<L>(
    mut treasurer: Treasurer<L>,
    mut latest_block_receiver: watch::Receiver<block::Block>,
) -> Result<()>
where
    L: Lcd + Clone + 'static,
{
    debug!("Init treasurer");

    while let Ok(()) = latest_block_receiver.changed().await {
        treasurer.control().await?;
    }

    Ok(())
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
pub enum Account {
    Arby,
    Treasury,
}

#[derive(Clone, Debug, Hash, Encode, Eq, PartialEq)]
struct Labels {
    account: Account,
}

pub struct Treasurer<L> {
    address_book: AddressBook,
    execute_trades: Arc<RwLock<bool>>,
    lcd: L,
    last: Option<Uint256>,

    account_balance_uusd: Family<Labels, Gauge>,
}

impl<L> Treasurer<L>
where
    L: Lcd + Clone + 'static,
{
    pub fn new(
        registry: &mut Registry,
        address_book: AddressBook,
        execute_trades: Arc<RwLock<bool>>,
        lcd: L,
    ) -> Self {
        let account_balance_uusd = Family::<Labels, Gauge>::default();
        registry.register(
            "account_balance_uusd",
            "Latest observed balance for a well-known account in uusd",
            Box::new(account_balance_uusd.clone()),
        );

        Self {
            address_book,
            execute_trades,
            lcd,
            last: None,

            account_balance_uusd,
        }
    }

    async fn control(&mut self) -> Result<()> {
        let arby_balances_handle = tokio::spawn(accounts::fetch(
            self.lcd.clone(),
            self.address_book.arby.clone(),
        ));
        let treasury_balances_handle = tokio::spawn(accounts::fetch(
            self.lcd.clone(),
            self.address_book.treasury.clone(),
        ));

        let (arby_balances, treasury_balances) = tokio::try_join!(
            flatten(arby_balances_handle),
            flatten(treasury_balances_handle)
        )?;

        let ust = Token {
            id: "uusd".to_owned(),
            decimals: 6,
            native: true,
        };
        let current = *arby_balances.get(&ust).unwrap_or(&Uint256::zero());
        let current_treasury = *treasury_balances.get(&ust).unwrap_or(&Uint256::zero());

        if let Some(last) = self.last {
            if last.multiply_ratio(9i64, 10i64) >= current {
                error!("balance critically dropped: {} -> {}", last, current);

                let execute_trades = Arc::clone(&self.execute_trades);
                tokio::task::spawn_blocking(move || {
                    let mut toggle = execute_trades.write().unwrap();
                    *toggle = false;
                })
                .await
                .unwrap();
            }
        }

        self.last = Some(current);

        self.account_balance_uusd
            .get_or_create(&Labels {
                account: Account::Arby,
            })
            .set(current.0.as_u64());

        self.account_balance_uusd
            .get_or_create(&Labels {
                account: Account::Treasury,
            })
            .set(current_treasury.0.as_u64());

        Ok(())
    }
}
