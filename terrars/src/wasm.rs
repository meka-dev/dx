use std::str::FromStr;

use common::Result;
// use chainrs::{ Any as Msg, MsgProto};
use cosmrs::tx::{Msg, MsgProto};
pub use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

use crate::proto;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryContractStoreResponse<T> {
    pub query_result: T,
}

#[derive(Clone, Debug)]
pub struct MsgExecuteContract {
    pub coins: Vec<chainrs::cosmos::base::Coin>,
    pub contract_address: Addr,
    pub execute_msg: Vec<u8>,
    pub sender: String,
}

impl Msg for MsgExecuteContract {
    type Proto = proto::terra::wasm::v1beta1::MsgExecuteContract;
}

impl From<&chainrs::cosmos::base::Coin> for proto::cosmos::base::v1beta1::Coin {
    fn from(value: &chainrs::cosmos::base::Coin) -> proto::cosmos::base::v1beta1::Coin {
        proto::cosmos::base::v1beta1::Coin {
            amount: value.amount.to_string(),
            denom: value.denom.to_string(),
        }
    }
}

impl TryFrom<&proto::cosmos::base::v1beta1::Coin> for chainrs::cosmos::base::Coin {
    type Error = cosmrs::ErrorReport;

    fn try_from(input: &proto::cosmos::base::v1beta1::Coin) -> Result<chainrs::cosmos::base::Coin> {
        Ok(chainrs::cosmos::base::Coin {
            amount: input.amount.parse::<cosmrs::Decimal>()?,
            denom: chainrs::cosmos::base::Denom::from_str(&input.denom)?,
        })
    }
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
