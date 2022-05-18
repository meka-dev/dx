use crate::{proto::cosmos::crisis, MsgProto};

impl MsgProto for crisis::v1beta1::MsgVerifyInvariant {
    const TYPE_URL: &'static str = "/cosmos.crisis.v1beta1.MsgVerifyInvariant";
}
