use common::{Report, Result};
use serde::{Deserialize, Serialize};

use crate::{
    cosmos::base::{query::PageResponse, Coin},
    proto::cosmos::bank,
    tx::{Msg, MsgProto},
    MsgProto as ChainMsgProto,
};

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct QueryAllBalancesResponse {
    pub balances: Vec<Coin>,
    pub pagination: PageResponse,
}

impl MsgProto for bank::v1beta1::MsgMultiSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgMultiSend";
}

impl MsgProto for bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl ChainMsgProto for bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}

impl Msg for MsgSend {
    type Proto = crate::proto::cosmos::bank::v1beta1::MsgSend;
}

/// MsgSend represents a message to send coins from one account to another.
#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct MsgSend {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Coin>,
}

impl From<MsgSend> for crate::proto::cosmos::bank::v1beta1::MsgSend {
    fn from(s: MsgSend) -> Self {
        crate::proto::cosmos::bank::v1beta1::MsgSend {
            from_address: s.from_address,
            to_address: s.to_address,
            amount: s.amount.into_iter().map(From::from).collect::<Vec<_>>(),
        }
    }
}

impl TryFrom<crate::proto::cosmos::bank::v1beta1::MsgSend> for MsgSend {
    type Error = Report;

    fn try_from(s: crate::proto::cosmos::bank::v1beta1::MsgSend) -> Result<Self> {
        Ok(MsgSend {
            from_address: s.from_address,
            to_address: s.to_address,
            amount: s
                .amount
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
