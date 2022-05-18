pub mod channel {
    use crate::{proto::ibc::core::channel, MsgProto};

    impl MsgProto for channel::v1::MsgAcknowledgement {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgAcknowledgement";
    }

    impl MsgProto for channel::v1::MsgChannelCloseConfirm {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgChannelCloseConfirm";
    }

    impl MsgProto for channel::v1::MsgChannelCloseInit {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgChannelCloseInit";
    }

    impl MsgProto for channel::v1::MsgChannelOpenAck {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgChannelOpenAck";
    }

    impl MsgProto for channel::v1::MsgChannelOpenConfirm {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgChannelOpenConfirm";
    }

    impl MsgProto for channel::v1::MsgChannelOpenInit {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgChannelOpenInit";
    }

    impl MsgProto for channel::v1::MsgChannelOpenTry {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgChannelOpenTry";
    }

    impl MsgProto for channel::v1::MsgRecvPacket {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgRecvPacket";
    }

    impl MsgProto for channel::v1::MsgTimeout {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgTimeout";
    }

    impl MsgProto for channel::v1::MsgTimeoutOnClose {
        const TYPE_URL: &'static str = "/ibc.core.channel.v1.MsgTimeoutOnClose";
    }
}

pub mod client {
    use crate::{proto::ibc::core::client, MsgProto};

    impl MsgProto for client::v1::MsgCreateClient {
        const TYPE_URL: &'static str = "/ibc.core.client.v1.MsgCreateClient";
    }

    impl MsgProto for client::v1::MsgSubmitMisbehaviour {
        const TYPE_URL: &'static str = "/ibc.core.client.v1.MsgSubmitMisbehaviour";
    }

    impl MsgProto for client::v1::MsgUpdateClient {
        const TYPE_URL: &'static str = "/ibc.core.client.v1.MsgUpdateClient";
    }

    impl MsgProto for client::v1::MsgUpgradeClient {
        const TYPE_URL: &'static str = "/ibc.core.client.v1.MsgUpgradeClient";
    }
}

pub mod connection {
    use crate::{proto::ibc::core::connection, MsgProto};

    impl MsgProto for connection::v1::MsgConnectionOpenAck {
        const TYPE_URL: &'static str = "/ibc.core.connection.v1.MsgConnectionOpenAck";
    }

    impl MsgProto for connection::v1::MsgConnectionOpenConfirm {
        const TYPE_URL: &'static str = "/ibc.core.connection.v1.MsgConnectionOpenConfirm";
    }

    impl MsgProto for connection::v1::MsgConnectionOpenInit {
        const TYPE_URL: &'static str = "/ibc.core.connection.v1.MsgConnectionOpenInit";
    }

    impl MsgProto for connection::v1::MsgConnectionOpenTry {
        const TYPE_URL: &'static str = "/ibc.core.connection.v1.MsgConnectionOpenTry";
    }
}
