use cosmrs::tx::MsgProto;

use crate::proto;

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgBurn {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.Msg/Burn";
}

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgChangeAdmin {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.Msg/ChangeAdmin";
}

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgCreateDenom {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.Msg/CreateDenom";
}

impl MsgProto for proto::osmosis::tokenfactory::v1beta1::MsgMint {
    const TYPE_URL: &'static str = "/osmosis.tokenfactory.v1beta1.Msg/Mint";
}
