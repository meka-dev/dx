use anyhow::Result;

use cosmrs::tx::MsgProto;
pub use cosmrs::{
    auth::BaseAccount,
    bank::MsgSend,
    crypto::secp256k1::SigningKey,
    tx::{Msg, Tx},
    AccountId, Any as Message, Decimal, Denom,
};
use cosmwasm_std::Addr;

mod bank;
mod base;
pub use base::Coin;
pub mod lcd;
mod proto;
pub mod tx;
mod wasm;

#[derive(Clone, Debug)]
pub struct MsgExecuteContract {
    pub coins: Vec<Coin>,
    pub contract_address: Addr,
    pub execute_msg: Vec<u8>,
    pub sender: String,
}

impl Msg for MsgExecuteContract {
    type Proto = proto::terra::wasm::v1beta1::MsgExecuteContract;
}

impl TryFrom<crate::proto::terra::wasm::v1beta1::MsgExecuteContract> for MsgExecuteContract {
    type Error = cosmrs::ErrorReport;

    fn try_from(
        input: proto::terra::wasm::v1beta1::MsgExecuteContract,
    ) -> cosmrs::Result<MsgExecuteContract> {
        Ok(MsgExecuteContract {
            sender: input.sender.parse()?,
            contract_address: Addr::unchecked(input.contract),
            execute_msg: input.execute_msg.into_iter().map(Into::into).collect(),
            coins: input
                .coins
                .iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgExecuteContract> for proto::terra::wasm::v1beta1::MsgExecuteContract {
    fn from(msg: MsgExecuteContract) -> proto::terra::wasm::v1beta1::MsgExecuteContract {
        crate::proto::terra::wasm::v1beta1::MsgExecuteContract {
            sender: msg.sender.to_string(),
            contract: msg.contract_address.to_string(),
            execute_msg: msg.execute_msg,
            coins: msg.coins.iter().map(Into::into).collect(),
        }
    }
}

impl MsgProto for proto::terra::wasm::v1beta1::MsgExecuteContract {
    const TYPE_URL: &'static str = "/terra.wasm.v1beta1.MsgExecuteContract";
}

#[derive(Clone, Debug, Default)]
pub struct QueryAccountRequest {
    pub address: String,
}

impl Msg for QueryAccountRequest {
    type Proto = proto::cosmos::auth::v1beta1::QueryAccountRequest;
}

impl TryFrom<crate::proto::cosmos::auth::v1beta1::QueryAccountRequest> for QueryAccountRequest {
    type Error = cosmrs::ErrorReport;

    fn try_from(
        input: proto::cosmos::auth::v1beta1::QueryAccountRequest,
    ) -> cosmrs::Result<QueryAccountRequest> {
        Ok(QueryAccountRequest {
            address: input.address,
        })
    }
}

impl From<QueryAccountRequest> for proto::cosmos::auth::v1beta1::QueryAccountRequest {
    fn from(query: QueryAccountRequest) -> proto::cosmos::auth::v1beta1::QueryAccountRequest {
        crate::proto::cosmos::auth::v1beta1::QueryAccountRequest {
            address: query.address,
        }
    }
}

impl MsgProto for proto::cosmos::auth::v1beta1::QueryAccountRequest {
    const TYPE_URL: &'static str = "/cosmos.auth.v1beta1.QueryAccountRequest";
}
