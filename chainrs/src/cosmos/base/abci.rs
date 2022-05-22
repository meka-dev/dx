use serde::{Deserialize, Serialize};

use crate::{
    json::{serialize_bytes_as_base64, serialize_field_as_string},
    proto::cosmos::base::abci,
    MsgProto,
};

impl MsgProto for abci::v1beta1::MsgData {
    const TYPE_URL: &'static str = "/cosmos.base.abci.v1beta1.MsgData";
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct GasInfo {
    #[serde(with = "serialize_field_as_string")]
    pub gas_wanted: u64,
    #[serde(with = "serialize_field_as_string")]
    pub gas_used: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Result {
    #[serde(with = "serialize_bytes_as_base64")]
    pub data: Vec<u8>,
    pub log: String,
    pub events: Vec<cosmrs::tendermint::abci::responses::Event>,
}
