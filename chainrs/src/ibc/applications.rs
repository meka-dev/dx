use crate::{proto::ibc::applications, MsgProto};

impl MsgProto for applications::transfer::v1::MsgTransfer {
    const TYPE_URL: &'static str = "/ibc.applications.transfer.v1.MsgTransfer";
}
