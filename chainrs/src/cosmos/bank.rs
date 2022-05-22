use serde::{Deserialize, Serialize};

use crate::{
    cosmos::base::{query::PageResponse, Coin},
    proto::cosmos::bank,
    MsgProto,
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
