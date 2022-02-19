use std::str::FromStr as _;

use cosmrs::tx::MsgProto;
pub use cosmrs::{
    crypto::secp256k1::SigningKey,
    tx::{Msg, Tx},
    AccountId, Any as Message, Decimal, Denom,
};
use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::Addr;

pub mod lcd;
mod proto;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Decimal256,
}

impl From<Coin> for cosmrs::Coin {
    fn from(input: Coin) -> Self {
        Self {
            denom: input.denom,
            amount: Decimal::from(input.amount.0.as_u64()),
        }
    }
}

impl From<terra_rust_api::core_types::Coin> for Coin {
    fn from(input: terra_rust_api::core_types::Coin) -> Self {
        Self {
            // FIXME(xla): Dangerous and slow.
            amount: Decimal256::from_str(&input.amount.to_string()).unwrap(),
            // XXX(xla): This should be a TryFrom.
            denom: Denom::from_str(&input.denom).unwrap(),
        }
    }
}

impl From<&Coin> for terra_rust_api::client::core_types::Coin {
    fn from(input: &Coin) -> Self {
        Self {
            // XXX(xla): These string allocations will mean severe performance degradation when
            // used in tight loops.
            amount: rust_decimal::Decimal::from_str(&input.amount.to_string()).unwrap(),
            denom: input.denom.to_string(),
        }
    }
}

impl TryFrom<proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = cosmrs::ErrorReport;

    fn try_from(input: proto::cosmos::base::v1beta1::Coin) -> cosmrs::Result<Self> {
        Coin::try_from(&input)
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for Coin {
    type Error = cosmrs::ErrorReport;

    fn try_from(input: &proto::cosmos::base::v1beta1::Coin) -> cosmrs::Result<Self> {
        Ok(Self {
            amount: input.amount.parse()?,
            denom: input.denom.parse()?,
        })
    }
}

impl From<Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin::from(&coin)
    }
}

impl From<&Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(coin: &Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin {
            denom: coin.denom.to_string(),
            amount: coin.amount.to_string(),
        }
    }
}

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
