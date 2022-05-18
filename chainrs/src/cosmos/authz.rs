use crate::{proto::cosmos::authz, MsgProto};

impl MsgProto for authz::v1beta1::MsgExec {
    const TYPE_URL: &'static str = "/cosmos.authz.v1beta1.MsgExec";
}

impl MsgProto for authz::v1beta1::MsgGrant {
    const TYPE_URL: &'static str = "/cosmos.authz.v1beta1.MsgGrant";
}

impl MsgProto for authz::v1beta1::MsgRevoke {
    const TYPE_URL: &'static str = "/cosmos.authz.v1beta1.MsgRevoke";
}
