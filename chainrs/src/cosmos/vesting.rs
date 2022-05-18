use crate::{proto::cosmos::vesting, MsgProto};

impl MsgProto for vesting::v1beta1::MsgCreateVestingAccount {
    const TYPE_URL: &'static str = "/cosmos.vesting.v1beta1.MsgBCreateVestingAccount";
}
