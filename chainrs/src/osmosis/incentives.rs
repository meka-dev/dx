use crate::{proto, MsgProto};

impl MsgProto for proto::osmosis::incentives::MsgAddToGauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.MsgAddToGauge";
}

impl MsgProto for proto::osmosis::incentives::MsgCreateGauge {
    const TYPE_URL: &'static str = "/osmosis.incentives.MsgCreateGauge";
}
