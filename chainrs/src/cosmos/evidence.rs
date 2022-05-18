use crate::{proto::cosmos::evidence, MsgProto};

impl MsgProto for evidence::v1beta1::MsgSubmitEvidence {
    const TYPE_URL: &'static str = "/cosmos.evidence.v1beta1.MsgSubmitEvidence";
}
