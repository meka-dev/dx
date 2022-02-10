use cosmrs::tx::MsgProto;
pub use cosmrs::{
    crypto::secp256k1::SigningKey,
    tx::{Msg, Tx},
    AccountId, Any as Message, Denom, Error,
};
use cosmwasm_bignumber::Decimal256;

pub use terra_rust_api::Terra as Lcd;

mod proto;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coin {
    /// Denomination
    pub denom: Denom,

    /// Amount
    pub amount: Decimal256,
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

#[derive(Clone, Debug, Default)]
pub struct MsgExecuteContract {
    pub coins: Vec<Coin>,
    pub contract: String,
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
            contract: input.contract.parse()?,
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
            contract: msg.contract.to_string(),
            execute_msg: msg.execute_msg,
            coins: msg.coins.iter().map(Into::into).collect(),
        }
    }
}

impl MsgProto for proto::terra::wasm::v1beta1::MsgExecuteContract {
    const TYPE_URL: &'static str = "/terra.wasm.v1beta1.MsgExecuteContract";
}
