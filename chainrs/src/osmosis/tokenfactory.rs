use crate::{proto, MsgProto};

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgBurn {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.MsgBurn";
}

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgChangeAdmin {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.MsgChangeAdmin";
}

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgCreateDenom {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.MsgCreateDenom";
}

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgMint {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.MsgMint";
}
