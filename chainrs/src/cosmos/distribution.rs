use crate::{proto::cosmos::distribution, MsgProto};

impl MsgProto for distribution::v1beta1::MsgFundCommunityPool {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgFundCommunityPool";
}

impl MsgProto for distribution::v1beta1::MsgSetWithdrawAddress {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress";
}

impl MsgProto for distribution::v1beta1::MsgWithdrawDelegatorReward {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward";
}

impl MsgProto for distribution::v1beta1::MsgWithdrawValidatorCommission {
    const TYPE_URL: &'static str = "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission";
}
