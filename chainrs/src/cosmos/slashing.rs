use crate::{proto::cosmos::slashing, MsgProto};

impl MsgProto for slashing::v1beta1::MsgUnjail {
    const TYPE_URL: &'static str = "/cosmos.slashing.v1beta1.MsgJail";
}
