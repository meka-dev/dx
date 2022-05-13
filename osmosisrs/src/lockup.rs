use cosmrs::tx::MsgProto;

use crate::proto;

impl MsgProto for proto::osmosis::lockup::MsgBeginUnlocking {
    const TYPE_URL: &'static str = "/osmosis.lockup.Msg/BeginUnlocking";
}

impl MsgProto for proto::osmosis::lockup::MsgBeginUnlockingAll {
    const TYPE_URL: &'static str = "/osmosis.lockup.Msg/BeginUnlockingAll";
}

impl MsgProto for proto::osmosis::lockup::MsgExtendLockup {
    const TYPE_URL: &'static str = "/osmosis.lockup.Msg/ExtendLockup";
}

impl MsgProto for proto::osmosis::lockup::MsgLockTokens {
    const TYPE_URL: &'static str = "/osmosis.lockup.Msg/LockTokens";
}
