use cosmrs::tx::MsgProto;

use crate::proto;

impl MsgProto for proto::osmosis::superfluid::MsgLockAndSuperfluidDelegate {
    const TYPE_URL: &'static str = "/osmosis.superfluid.Msg/LockAndSuperfluidDelegate";
}

impl MsgProto for proto::osmosis::superfluid::MsgSuperfluidDelegate {
    const TYPE_URL: &'static str = "/osmosis.superfluid.Msg/SuperfluidDelegate";
}

impl MsgProto for proto::osmosis::superfluid::MsgSuperfluidUnbondLock {
    const TYPE_URL: &'static str = "/osmosis.superfluid.Msg/SuperfluidUnboundLock";
}

impl MsgProto for proto::osmosis::superfluid::MsgSuperfluidUndelegate {
    const TYPE_URL: &'static str = "/osmosis.superfluid.Msg/SuperfluidUndelegate";
}
