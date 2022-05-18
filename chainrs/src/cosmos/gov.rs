use crate::{proto::cosmos::gov, MsgProto};

impl MsgProto for gov::v1beta1::MsgDeposit {
    const TYPE_URL: &'static str = "/cosmos.gov.v1beta1.MsgDeposit";
}

impl MsgProto for gov::v1beta1::MsgSubmitProposal {
    const TYPE_URL: &'static str = "/cosmos.gov.v1beta1.MsgSubmitProposal";
}

impl MsgProto for gov::v1beta1::MsgVote {
    const TYPE_URL: &'static str = "/cosmos.gov.v1beta1.MsgVote";
}

impl MsgProto for gov::v1beta1::MsgVoteWeighted {
    const TYPE_URL: &'static str = "/cosmos.gov.v1beta1.MsgVoteWeighted";
}
