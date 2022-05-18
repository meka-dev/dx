use crate::{proto::cosmos::staking, MsgProto};

impl MsgProto for staking::v1beta1::MsgBeginRedelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgBeginRedelegate";
}

impl MsgProto for staking::v1beta1::MsgCreateValidator {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgCreateValidator";
}

impl MsgProto for staking::v1beta1::MsgDelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgDelegate";
}

impl MsgProto for staking::v1beta1::MsgEditValidator {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgEditValidator";
}

impl MsgProto for staking::v1beta1::MsgUndelegate {
    const TYPE_URL: &'static str = "/cosmos.staking.v1beta1.MsgUndelegate";
}
