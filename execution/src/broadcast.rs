use async_trait::async_trait;
use chainrs::{
    cosmos::{auth::BaseAccount, base::AccountId},
    proto::cosmos::base::abci::v1beta1::GasInfo,
};
use common::Result;
use strategy::Trade;
use tendermint::chain;
use tendermint_rpc::{endpoint::broadcast, WebSocketClient};

pub type Fee = u64;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Profitable,
    NotProfitable,
}

#[derive(Debug)]
pub struct Simulation {
    pub fee: u64,
    pub gas_info: GasInfo,
    pub outcome: Outcome,
}

#[async_trait]
pub trait Broadcaster {
    async fn broadcast(
        &mut self,
        fee: Fee,
        gas_info: GasInfo,
        trade: &Trade,
    ) -> Result<broadcast::tx_sync::Response>;

    async fn simulate(&self, trade: &Trade) -> Result<Simulation>;
}

pub struct Rpc {
    pub account: BaseAccount,
    pub client: WebSocketClient,
    pub chain_id: chain::Id,
    pub signing_key_bytes: Vec<u8>,
    pub sequence: u64,
    pub treasury: Option<AccountId>,
    pub address_book: address_book::AddressBook,
}

impl Rpc {
    pub fn new(
        client: WebSocketClient,
        chain_id: chain::Id,
        account: BaseAccount,
        signing_key_bytes: Vec<u8>,
        treasury: Option<AccountId>,
        address_book: address_book::AddressBook,
    ) -> Self {
        let sequence = account.sequence;

        Self {
            account,
            chain_id,
            client,
            signing_key_bytes,
            sequence,
            treasury,
            address_book,
        }
    }
}
