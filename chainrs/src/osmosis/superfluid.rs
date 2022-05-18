use crate::{proto::osmosis::superfluid, MsgProto};

impl MsgProto for superfluid::MsgLockAndSuperfluidDelegate {
    const TYPE_URL: &'static str = "/osmosis.superfluid.MsgLockAndSuperfluidDelegate";
}

impl MsgProto for superfluid::MsgSuperfluidDelegate {
    const TYPE_URL: &'static str = "/osmosis.superfluid.MsgSuperfluidDelegate";
}

impl MsgProto for superfluid::MsgSuperfluidUnbondLock {
    const TYPE_URL: &'static str = "/osmosis.superfluid.MsgSuperfluidUnbondLock";
}

impl MsgProto for superfluid::MsgSuperfluidUndelegate {
    const TYPE_URL: &'static str = "/osmosis.superfluid.MsgSuperfluidUndelegate";
}

impl MsgProto for superfluid::MsgUnPoolWhitelistedPool {
    const TYPE_URL: &'static str = "/osmosis.superfluid.MsgUnPoolWhitelistedPool";
}
