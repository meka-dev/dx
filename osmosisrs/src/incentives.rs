use cosmrs::tx::MsgProto;

use crate::proto;

impl MsgProto for proto::osmosis::incentives::MsgAddToGauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.Msg/AddToGauge";
}

impl MsgProto for proto::osmosis::incentives::MsgCreateGauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.Msg/CreateGauge";
}
