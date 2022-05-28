/// Channel defines pipeline for exactly-once packet delivery between specific
/// modules on separate blockchains, which has at least one end capable of
/// sending packets and one end capable of receiving packets.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// current state of the channel end
    #[prost(enumeration="State", tag="1")]
    pub state: i32,
    /// whether the channel is ordered or unordered
    #[prost(enumeration="Order", tag="2")]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag="3")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// list of connection identifiers, in order, along which packets sent on
    /// this channel will travel
    #[prost(string, repeated, tag="4")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// opaque channel version, which is agreed upon during the handshake
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
}
/// IdentifiedChannel defines a channel with additional port and channel
/// identifier fields.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedChannel {
    /// current state of the channel end
    #[prost(enumeration="State", tag="1")]
    pub state: i32,
    /// whether the channel is ordered or unordered
    #[prost(enumeration="Order", tag="2")]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag="3")]
    pub counterparty: ::core::option::Option<Counterparty>,
    /// list of connection identifiers, in order, along which packets sent on
    /// this channel will travel
    #[prost(string, repeated, tag="4")]
    pub connection_hops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// opaque channel version, which is agreed upon during the handshake
    #[prost(string, tag="5")]
    pub version: ::prost::alloc::string::String,
    /// port identifier
    #[prost(string, tag="6")]
    pub port_id: ::prost::alloc::string::String,
    /// channel identifier
    #[prost(string, tag="7")]
    pub channel_id: ::prost::alloc::string::String,
}
/// Counterparty defines a channel end counterparty
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Counterparty {
    /// port on the counterparty chain which owns the other end of the channel.
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel end on the counterparty chain
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
}
/// Packet defines a type that carries data across different chains through IBC
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    /// number corresponds to the order of sends and receives, where a Packet
    /// with an earlier sequence number must be sent and received before a Packet
    /// with a later sequence number.
    #[prost(uint64, tag="1")]
    pub sequence: u64,
    /// identifies the port on the sending chain.
    #[prost(string, tag="2")]
    pub source_port: ::prost::alloc::string::String,
    /// identifies the channel end on the sending chain.
    #[prost(string, tag="3")]
    pub source_channel: ::prost::alloc::string::String,
    /// identifies the port on the receiving chain.
    #[prost(string, tag="4")]
    pub destination_port: ::prost::alloc::string::String,
    /// identifies the channel end on the receiving chain.
    #[prost(string, tag="5")]
    pub destination_channel: ::prost::alloc::string::String,
    /// actual opaque bytes transferred directly to the application module
    #[prost(bytes="vec", tag="6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// block height after which the packet times out
    #[prost(message, optional, tag="7")]
    pub timeout_height: ::core::option::Option<super::super::client::v1::Height>,
    /// block timestamp (in nanoseconds) after which the packet times out
    #[prost(uint64, tag="8")]
    pub timeout_timestamp: u64,
}
/// PacketState defines the generic type necessary to retrieve and store
/// packet commitments, acknowledgements, and receipts.
/// Caller is responsible for knowing the context necessary to interpret this
/// state as a commitment, acknowledgement, or a receipt.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketState {
    /// channel port identifier.
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier.
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence.
    #[prost(uint64, tag="3")]
    pub sequence: u64,
    /// embedded data that represents packet state.
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Acknowledgement is the recommended acknowledgement format to be used by
/// app-specific protocols.
/// NOTE: The field numbers 21 and 22 were explicitly chosen to avoid accidental
/// conflicts with other protobuf message formats used for acknowledgements.
/// The first byte of any message with this format will be the non-ASCII values
/// `0xaa` (result) or `0xb2` (error). Implemented as defined by ICS:
/// <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#acknowledgement-envelope>
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acknowledgement {
    /// response contains either a result or an error and must be non-empty
    #[prost(oneof="acknowledgement::Response", tags="21, 22")]
    pub response: ::core::option::Option<acknowledgement::Response>,
}
/// Nested message and enum types in `Acknowledgement`.
pub mod acknowledgement {
    /// response contains either a result or an error and must be non-empty
    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(bytes, tag="21")]
        Result(::prost::alloc::vec::Vec<u8>),
        #[prost(string, tag="22")]
        Error(::prost::alloc::string::String),
    }
}
/// State defines if a channel is in one of the following states:
/// CLOSED, INIT, TRYOPEN, OPEN or UNINITIALIZED.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// Default State
    UninitializedUnspecified = 0,
    /// A channel has just started the opening handshake.
    Init = 1,
    /// A channel has acknowledged the handshake step on the counterparty chain.
    Tryopen = 2,
    /// A channel has completed the handshake. Open channels are
    /// ready to send and receive packets.
    Open = 3,
    /// A channel has been closed and can no longer be used to send or receive
    /// packets.
    Closed = 4,
}
/// Order defines if a channel is ORDERED or UNORDERED
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Order {
    /// zero-value for channel ordering
    NoneUnspecified = 0,
    /// packets can be delivered in any order, which may differ from the order in
    /// which they were sent.
    Unordered = 1,
    /// packets are delivered exactly in the order which they were sent
    Ordered = 2,
}
/// MsgChannelOpenInit defines an sdk.Msg to initialize a channel handshake. It
/// is called by a relayer on Chain A.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenInit {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenInitResponse defines the Msg/ChannelOpenInit response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenInitResponse {
}
/// MsgChannelOpenInit defines a msg sent by a Relayer to try to open a channel
/// on Chain B.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenTry {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// in the case of crossing hello's, when both chains call OpenInit, we need
    /// the channel identifier of the previous channel in state INIT
    #[prost(string, tag="2")]
    pub previous_channel_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub channel: ::core::option::Option<Channel>,
    #[prost(string, tag="4")]
    pub counterparty_version: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="7")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenTryResponse defines the Msg/ChannelOpenTry response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenTryResponse {
}
/// MsgChannelOpenAck defines a msg sent by a Relayer to Chain A to acknowledge
/// the change of channel state to TRYOPEN on Chain B.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenAck {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub counterparty_channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub counterparty_version: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub proof_try: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="6")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="7")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenAckResponse defines the Msg/ChannelOpenAck response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenAckResponse {
}
/// MsgChannelOpenConfirm defines a msg sent by a Relayer to Chain B to
/// acknowledge the change of channel state to OPEN on Chain A.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenConfirm {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub proof_ack: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelOpenConfirmResponse defines the Msg/ChannelOpenConfirm response
/// type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelOpenConfirmResponse {
}
/// MsgChannelCloseInit defines a msg sent by a Relayer to Chain A
/// to close a channel with Chain B.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseInit {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelCloseInitResponse defines the Msg/ChannelCloseInit response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseInitResponse {
}
/// MsgChannelCloseConfirm defines a msg sent by a Relayer to Chain B
/// to acknowledge the change of channel state to CLOSED on Chain A.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseConfirm {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub proof_init: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgChannelCloseConfirmResponse defines the Msg/ChannelCloseConfirm response
/// type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChannelCloseConfirmResponse {
}
/// MsgRecvPacket receives incoming IBC packet
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecvPacket {
    #[prost(message, optional, tag="1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes="vec", tag="2")]
    pub proof_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRecvPacketResponse defines the Msg/RecvPacket response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecvPacketResponse {
}
/// MsgTimeout receives timed-out packet
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeout {
    #[prost(message, optional, tag="1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes="vec", tag="2")]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(uint64, tag="4")]
    pub next_sequence_recv: u64,
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgTimeoutResponse defines the Msg/Timeout response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeoutResponse {
}
/// MsgTimeoutOnClose timed-out packet upon counterparty channel closure.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeoutOnClose {
    #[prost(message, optional, tag="1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes="vec", tag="2")]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub proof_close: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(uint64, tag="5")]
    pub next_sequence_recv: u64,
    #[prost(string, tag="6")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgTimeoutOnCloseResponse defines the Msg/TimeoutOnClose response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTimeoutOnCloseResponse {
}
/// MsgAcknowledgement receives incoming IBC acknowledgement
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcknowledgement {
    #[prost(message, optional, tag="1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes="vec", tag="2")]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub proof_acked: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag="5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAcknowledgementResponse defines the Msg/Acknowledgement response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAcknowledgementResponse {
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Msg defines the ibc/channel Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// ChannelOpenInit defines a rpc handler method for MsgChannelOpenInit.
        pub async fn channel_open_init(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenInit>,
        ) -> Result<tonic::Response<super::MsgChannelOpenInitResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenInit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelOpenTry defines a rpc handler method for MsgChannelOpenTry.
        pub async fn channel_open_try(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenTry>,
        ) -> Result<tonic::Response<super::MsgChannelOpenTryResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenTry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelOpenAck defines a rpc handler method for MsgChannelOpenAck.
        pub async fn channel_open_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenAck>,
        ) -> Result<tonic::Response<super::MsgChannelOpenAckResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenAck",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelOpenConfirm defines a rpc handler method for MsgChannelOpenConfirm.
        pub async fn channel_open_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelOpenConfirm>,
        ) -> Result<
                tonic::Response<super::MsgChannelOpenConfirmResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelOpenConfirm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelCloseInit defines a rpc handler method for MsgChannelCloseInit.
        pub async fn channel_close_init(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelCloseInit>,
        ) -> Result<tonic::Response<super::MsgChannelCloseInitResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelCloseInit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelCloseConfirm defines a rpc handler method for
        /// MsgChannelCloseConfirm.
        pub async fn channel_close_confirm(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgChannelCloseConfirm>,
        ) -> Result<
                tonic::Response<super::MsgChannelCloseConfirmResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/ChannelCloseConfirm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RecvPacket defines a rpc handler method for MsgRecvPacket.
        pub async fn recv_packet(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRecvPacket>,
        ) -> Result<tonic::Response<super::MsgRecvPacketResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/RecvPacket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Timeout defines a rpc handler method for MsgTimeout.
        pub async fn timeout(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTimeout>,
        ) -> Result<tonic::Response<super::MsgTimeoutResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/Timeout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// TimeoutOnClose defines a rpc handler method for MsgTimeoutOnClose.
        pub async fn timeout_on_close(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTimeoutOnClose>,
        ) -> Result<tonic::Response<super::MsgTimeoutOnCloseResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/TimeoutOnClose",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Acknowledgement defines a rpc handler method for MsgAcknowledgement.
        pub async fn acknowledgement(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAcknowledgement>,
        ) -> Result<tonic::Response<super::MsgAcknowledgementResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Msg/Acknowledgement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// QueryChannelRequest is the request type for the Query/Channel RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryChannelResponse is the response type for the Query/Channel RPC method.
/// Besides the Channel end, it includes a proof and the height from which the
/// proof was retrieved.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelResponse {
    /// channel associated with the request identifiers
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<Channel>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryChannelsRequest is the request type for the Query/Channels RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelsRequest {
    /// pagination request
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryChannelsResponse is the response type for the Query/Channels RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelsResponse {
    /// list of stored channels of the chain.
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
    /// query block height
    #[prost(message, optional, tag="3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryConnectionChannelsRequest is the request type for the
/// Query/QueryConnectionChannels RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionChannelsRequest {
    /// connection unique identifier
    #[prost(string, tag="1")]
    pub connection: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryConnectionChannelsResponse is the Response type for the
/// Query/QueryConnectionChannels RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConnectionChannelsResponse {
    /// list of channels associated with a connection.
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
    /// query block height
    #[prost(message, optional, tag="3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryChannelClientStateRequest is the request type for the Query/ClientState
/// RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelClientStateRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryChannelClientStateResponse is the Response type for the
/// Query/QueryChannelClientState RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelClientStateResponse {
    /// client state associated with the channel
    #[prost(message, optional, tag="1")]
    pub identified_client_state: ::core::option::Option<super::super::client::v1::IdentifiedClientState>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryChannelConsensusStateRequest is the request type for the
/// Query/ConsensusState RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelConsensusStateRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// revision number of the consensus state
    #[prost(uint64, tag="3")]
    pub revision_number: u64,
    /// revision height of the consensus state
    #[prost(uint64, tag="4")]
    pub revision_height: u64,
}
/// QueryChannelClientStateResponse is the Response type for the
/// Query/QueryChannelClientState RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChannelConsensusStateResponse {
    /// consensus state associated with the channel
    #[prost(message, optional, tag="1")]
    pub consensus_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// client ID associated with the consensus state
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketCommitmentRequest is the request type for the
/// Query/PacketCommitment RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag="3")]
    pub sequence: u64,
}
/// QueryPacketCommitmentResponse defines the client query response for a packet
/// which also includes a proof and the height from which the proof was
/// retrieved
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentResponse {
    /// packet associated with the request fields
    #[prost(bytes="vec", tag="1")]
    pub commitment: ::prost::alloc::vec::Vec<u8>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketCommitmentsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentsRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag="3")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPacketCommitmentsResponse is the request type for the
/// Query/QueryPacketCommitments RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketCommitmentsResponse {
    #[prost(message, repeated, tag="1")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
    /// query block height
    #[prost(message, optional, tag="3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketReceiptRequest is the request type for the
/// Query/PacketReceipt RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketReceiptRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag="3")]
    pub sequence: u64,
}
/// QueryPacketReceiptResponse defines the client query response for a packet
/// receipt which also includes a proof, and the height from which the proof was
/// retrieved
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketReceiptResponse {
    /// success flag for if receipt exists
    #[prost(bool, tag="2")]
    pub received: bool,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketAcknowledgementRequest is the request type for the
/// Query/PacketAcknowledgement RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag="3")]
    pub sequence: u64,
}
/// QueryPacketAcknowledgementResponse defines the client query response for a
/// packet which also includes a proof and the height from which the
/// proof was retrieved
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementResponse {
    /// packet associated with the request fields
    #[prost(bytes="vec", tag="1")]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketAcknowledgementsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementsRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag="3")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    /// list of packet sequences
    #[prost(uint64, repeated, tag="4")]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryPacketAcknowledgemetsResponse is the request type for the
/// Query/QueryPacketAcknowledgements RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPacketAcknowledgementsResponse {
    #[prost(message, repeated, tag="1")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
    /// query block height
    #[prost(message, optional, tag="3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryUnreceivedPacketsRequest is the request type for the
/// Query/UnreceivedPackets RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedPacketsRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// list of packet sequences
    #[prost(uint64, repeated, tag="3")]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryUnreceivedPacketsResponse is the response type for the
/// Query/UnreceivedPacketCommitments RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedPacketsResponse {
    /// list of unreceived packet sequences
    #[prost(uint64, repeated, tag="1")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag="2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryUnreceivedAcks is the request type for the
/// Query/UnreceivedAcks RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedAcksRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    /// list of acknowledgement sequences
    #[prost(uint64, repeated, tag="3")]
    pub packet_ack_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryUnreceivedAcksResponse is the response type for the
/// Query/UnreceivedAcks RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUnreceivedAcksResponse {
    /// list of unreceived acknowledgement sequences
    #[prost(uint64, repeated, tag="1")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag="2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryNextSequenceReceiveRequest is the request type for the
/// Query/QueryNextSequenceReceiveRequest RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextSequenceReceiveRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// channel unique identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QuerySequenceResponse is the request type for the
/// Query/QueryNextSequenceReceiveResponse RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNextSequenceReceiveResponse {
    /// next sequence receive number
    #[prost(uint64, tag="1")]
    pub next_sequence_receive: u64,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Query provides defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Channel queries an IBC Channel.
        pub async fn channel(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelRequest>,
        ) -> Result<tonic::Response<super::QueryChannelResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/Channel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Channels queries all the IBC channels of a chain.
        pub async fn channels(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelsRequest>,
        ) -> Result<tonic::Response<super::QueryChannelsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/Channels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ConnectionChannels queries all the channels associated with a connection
        /// end.
        pub async fn connection_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConnectionChannelsRequest>,
        ) -> Result<
                tonic::Response<super::QueryConnectionChannelsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ConnectionChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelClientState queries for the client state for the channel associated
        /// with the provided channel identifiers.
        pub async fn channel_client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelClientStateRequest>,
        ) -> Result<
                tonic::Response<super::QueryChannelClientStateResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ChannelClientState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ChannelConsensusState queries for the consensus state for the channel
        /// associated with the provided channel identifiers.
        pub async fn channel_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryChannelConsensusStateRequest>,
        ) -> Result<
                tonic::Response<super::QueryChannelConsensusStateResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/ChannelConsensusState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PacketCommitment queries a stored packet commitment hash.
        pub async fn packet_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketCommitmentRequest>,
        ) -> Result<
                tonic::Response<super::QueryPacketCommitmentResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketCommitment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PacketCommitments returns all the packet commitments hashes associated
        /// with a channel.
        pub async fn packet_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketCommitmentsRequest>,
        ) -> Result<
                tonic::Response<super::QueryPacketCommitmentsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketCommitments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PacketReceipt queries if a given packet sequence has been received on the
        /// queried chain
        pub async fn packet_receipt(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketReceiptRequest>,
        ) -> Result<tonic::Response<super::QueryPacketReceiptResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketReceipt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PacketAcknowledgement queries a stored packet acknowledgement hash.
        pub async fn packet_acknowledgement(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketAcknowledgementRequest>,
        ) -> Result<
                tonic::Response<super::QueryPacketAcknowledgementResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketAcknowledgement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PacketAcknowledgements returns all the packet acknowledgements associated
        /// with a channel.
        pub async fn packet_acknowledgements(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPacketAcknowledgementsRequest>,
        ) -> Result<
                tonic::Response<super::QueryPacketAcknowledgementsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/PacketAcknowledgements",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UnreceivedPackets returns all the unreceived IBC packets associated with a
        /// channel and sequences.
        pub async fn unreceived_packets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnreceivedPacketsRequest>,
        ) -> Result<
                tonic::Response<super::QueryUnreceivedPacketsResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/UnreceivedPackets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UnreceivedAcks returns all the unreceived IBC acknowledgements associated
        /// with a channel and sequences.
        pub async fn unreceived_acks(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUnreceivedAcksRequest>,
        ) -> Result<tonic::Response<super::QueryUnreceivedAcksResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/UnreceivedAcks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// NextSequenceReceive returns the next receive sequence for a given channel.
        pub async fn next_sequence_receive(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNextSequenceReceiveRequest>,
        ) -> Result<
                tonic::Response<super::QueryNextSequenceReceiveResponse>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.channel.v1.Query/NextSequenceReceive",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the ibc channel submodule's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub channels: ::prost::alloc::vec::Vec<IdentifiedChannel>,
    #[prost(message, repeated, tag="2")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag="3")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag="4")]
    pub receipts: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag="5")]
    pub send_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    #[prost(message, repeated, tag="6")]
    pub recv_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    #[prost(message, repeated, tag="7")]
    pub ack_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
    /// the sequence for the next generated channel identifier
    #[prost(uint64, tag="8")]
    pub next_channel_sequence: u64,
}
/// PacketSequence defines the genesis type necessary to retrieve and store
/// next send and receive sequences.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketSequence {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub sequence: u64,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CHANNEL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.Channel")] impl :: prost_wkt :: MessageSerde for Channel { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "Channel" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.Channel" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_IDENTIFIED_CHANNEL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.IdentifiedChannel")] impl :: prost_wkt :: MessageSerde for IdentifiedChannel { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "IdentifiedChannel" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.IdentifiedChannel" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_COUNTERPARTY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.Counterparty")] impl :: prost_wkt :: MessageSerde for Counterparty { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "Counterparty" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.Counterparty" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PACKET : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.Packet")] impl :: prost_wkt :: MessageSerde for Packet { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "Packet" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.Packet" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PACKET_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.PacketState")] impl :: prost_wkt :: MessageSerde for PacketState { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "PacketState" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.PacketState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACKNOWLEDGEMENT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.Acknowledgement")] impl :: prost_wkt :: MessageSerde for Acknowledgement { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "Acknowledgement" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.Acknowledgement" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_INIT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenInit")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenInit { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenInit" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenInit" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_INIT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenInitResponse")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenInitResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenInitResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenInitResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_TRY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenTry")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenTry { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenTry" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenTry" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_TRY_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenTryResponse")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenTryResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenTryResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenTryResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_ACK : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenAck")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenAck { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenAck" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenAck" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_ACK_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenAckResponse")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenAckResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenAckResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenAckResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_CONFIRM : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenConfirm")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenConfirm { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenConfirm" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenConfirm" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_OPEN_CONFIRM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelOpenConfirmResponse")] impl :: prost_wkt :: MessageSerde for MsgChannelOpenConfirmResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelOpenConfirmResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelOpenConfirmResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_CLOSE_INIT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelCloseInit")] impl :: prost_wkt :: MessageSerde for MsgChannelCloseInit { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelCloseInit" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelCloseInit" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_CLOSE_INIT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelCloseInitResponse")] impl :: prost_wkt :: MessageSerde for MsgChannelCloseInitResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelCloseInitResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelCloseInitResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_CLOSE_CONFIRM : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelCloseConfirm")] impl :: prost_wkt :: MessageSerde for MsgChannelCloseConfirm { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelCloseConfirm" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelCloseConfirm" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CHANNEL_CLOSE_CONFIRM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgChannelCloseConfirmResponse")] impl :: prost_wkt :: MessageSerde for MsgChannelCloseConfirmResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgChannelCloseConfirmResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgChannelCloseConfirmResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_RECV_PACKET : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgRecvPacket")] impl :: prost_wkt :: MessageSerde for MsgRecvPacket { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgRecvPacket" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgRecvPacket" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_RECV_PACKET_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgRecvPacketResponse")] impl :: prost_wkt :: MessageSerde for MsgRecvPacketResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgRecvPacketResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgRecvPacketResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_TIMEOUT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgTimeout")] impl :: prost_wkt :: MessageSerde for MsgTimeout { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgTimeout" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgTimeout" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_TIMEOUT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgTimeoutResponse")] impl :: prost_wkt :: MessageSerde for MsgTimeoutResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgTimeoutResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgTimeoutResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_TIMEOUT_ON_CLOSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgTimeoutOnClose")] impl :: prost_wkt :: MessageSerde for MsgTimeoutOnClose { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgTimeoutOnClose" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgTimeoutOnClose" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_TIMEOUT_ON_CLOSE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgTimeoutOnCloseResponse")] impl :: prost_wkt :: MessageSerde for MsgTimeoutOnCloseResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgTimeoutOnCloseResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgTimeoutOnCloseResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_ACKNOWLEDGEMENT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgAcknowledgement")] impl :: prost_wkt :: MessageSerde for MsgAcknowledgement { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgAcknowledgement" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgAcknowledgement" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_ACKNOWLEDGEMENT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.MsgAcknowledgementResponse")] impl :: prost_wkt :: MessageSerde for MsgAcknowledgementResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "MsgAcknowledgementResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.MsgAcknowledgementResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNEL_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelRequest")] impl :: prost_wkt :: MessageSerde for QueryChannelRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNEL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelResponse")] impl :: prost_wkt :: MessageSerde for QueryChannelResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNELS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelsRequest")] impl :: prost_wkt :: MessageSerde for QueryChannelsRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelsRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNELS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelsResponse")] impl :: prost_wkt :: MessageSerde for QueryChannelsResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelsResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONNECTION_CHANNELS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryConnectionChannelsRequest")] impl :: prost_wkt :: MessageSerde for QueryConnectionChannelsRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryConnectionChannelsRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryConnectionChannelsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONNECTION_CHANNELS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryConnectionChannelsResponse")] impl :: prost_wkt :: MessageSerde for QueryConnectionChannelsResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryConnectionChannelsResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryConnectionChannelsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNEL_CLIENT_STATE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelClientStateRequest")] impl :: prost_wkt :: MessageSerde for QueryChannelClientStateRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelClientStateRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelClientStateRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNEL_CLIENT_STATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelClientStateResponse")] impl :: prost_wkt :: MessageSerde for QueryChannelClientStateResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelClientStateResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelClientStateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNEL_CONSENSUS_STATE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelConsensusStateRequest")] impl :: prost_wkt :: MessageSerde for QueryChannelConsensusStateRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelConsensusStateRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelConsensusStateRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CHANNEL_CONSENSUS_STATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryChannelConsensusStateResponse")] impl :: prost_wkt :: MessageSerde for QueryChannelConsensusStateResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryChannelConsensusStateResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryChannelConsensusStateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_COMMITMENT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketCommitmentRequest")] impl :: prost_wkt :: MessageSerde for QueryPacketCommitmentRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketCommitmentRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketCommitmentRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_COMMITMENT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketCommitmentResponse")] impl :: prost_wkt :: MessageSerde for QueryPacketCommitmentResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketCommitmentResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketCommitmentResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_COMMITMENTS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketCommitmentsRequest")] impl :: prost_wkt :: MessageSerde for QueryPacketCommitmentsRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketCommitmentsRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketCommitmentsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_COMMITMENTS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketCommitmentsResponse")] impl :: prost_wkt :: MessageSerde for QueryPacketCommitmentsResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketCommitmentsResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketCommitmentsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_RECEIPT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketReceiptRequest")] impl :: prost_wkt :: MessageSerde for QueryPacketReceiptRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketReceiptRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketReceiptRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_RECEIPT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketReceiptResponse")] impl :: prost_wkt :: MessageSerde for QueryPacketReceiptResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketReceiptResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketReceiptResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_ACKNOWLEDGEMENT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketAcknowledgementRequest")] impl :: prost_wkt :: MessageSerde for QueryPacketAcknowledgementRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketAcknowledgementRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketAcknowledgementRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_ACKNOWLEDGEMENT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketAcknowledgementResponse")] impl :: prost_wkt :: MessageSerde for QueryPacketAcknowledgementResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketAcknowledgementResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketAcknowledgementResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_ACKNOWLEDGEMENTS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketAcknowledgementsRequest")] impl :: prost_wkt :: MessageSerde for QueryPacketAcknowledgementsRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketAcknowledgementsRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketAcknowledgementsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PACKET_ACKNOWLEDGEMENTS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryPacketAcknowledgementsResponse")] impl :: prost_wkt :: MessageSerde for QueryPacketAcknowledgementsResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryPacketAcknowledgementsResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryPacketAcknowledgementsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UNRECEIVED_PACKETS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryUnreceivedPacketsRequest")] impl :: prost_wkt :: MessageSerde for QueryUnreceivedPacketsRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryUnreceivedPacketsRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryUnreceivedPacketsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UNRECEIVED_PACKETS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryUnreceivedPacketsResponse")] impl :: prost_wkt :: MessageSerde for QueryUnreceivedPacketsResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryUnreceivedPacketsResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryUnreceivedPacketsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UNRECEIVED_ACKS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryUnreceivedAcksRequest")] impl :: prost_wkt :: MessageSerde for QueryUnreceivedAcksRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryUnreceivedAcksRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryUnreceivedAcksRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UNRECEIVED_ACKS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryUnreceivedAcksResponse")] impl :: prost_wkt :: MessageSerde for QueryUnreceivedAcksResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryUnreceivedAcksResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryUnreceivedAcksResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_NEXT_SEQUENCE_RECEIVE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryNextSequenceReceiveRequest")] impl :: prost_wkt :: MessageSerde for QueryNextSequenceReceiveRequest { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryNextSequenceReceiveRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryNextSequenceReceiveRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_NEXT_SEQUENCE_RECEIVE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.QueryNextSequenceReceiveResponse")] impl :: prost_wkt :: MessageSerde for QueryNextSequenceReceiveResponse { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "QueryNextSequenceReceiveResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.QueryNextSequenceReceiveResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PACKET_SEQUENCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.channel.v1.PacketSequence")] impl :: prost_wkt :: MessageSerde for PacketSequence { fn package_name (& self) -> & 'static str { "ibc.core.channel.v1" } fn message_name (& self) -> & 'static str { "PacketSequence" } fn type_url (& self) -> & 'static str { "/ibc.core.channel.v1.PacketSequence" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
