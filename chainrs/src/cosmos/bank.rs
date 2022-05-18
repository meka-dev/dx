use crate::{proto::cosmos::bank, MsgProto};

impl MsgProto for bank::v1beta1::MsgMultiSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgMultiSend";
}

impl MsgProto for bank::v1beta1::MsgSend {
    const TYPE_URL: &'static str = "/cosmos.bank.v1beta1.MsgSend";
}
