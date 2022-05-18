use crate::{proto, MsgProto};

impl MsgProto for proto::osmosis::lockup::MsgBeginUnlocking {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgBeginUnlocking";
}

impl MsgProto for proto::osmosis::lockup::MsgBeginUnlockingAll {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgBeginUnlockingAll";
}

// impl MsgProto for proto::osmosis::lockup::MsgExtendLockup {
//     const TYPE_URL: &'static str = "/osmosis.lockup.MsgExtendLockup";
// }

impl MsgProto for proto::osmosis::lockup::MsgLockTokens {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgLockTokens";
}
