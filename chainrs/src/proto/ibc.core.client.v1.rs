/// IdentifiedClientState defines a client state with an additional client
/// identifier field.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedClientState {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// client state
    #[prost(message, optional, tag="2")]
    pub client_state: ::core::option::Option<::prost_wkt_types::Any>,
}
/// ConsensusStateWithHeight defines a consensus state with an additional height
/// field.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusStateWithHeight {
    /// consensus state height
    #[prost(message, optional, tag="1")]
    pub height: ::core::option::Option<Height>,
    /// consensus state
    #[prost(message, optional, tag="2")]
    pub consensus_state: ::core::option::Option<::prost_wkt_types::Any>,
}
/// ClientConsensusStates defines all the stored consensus states for a given
/// client.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConsensusStates {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus states and their heights associated with the client
    #[prost(message, repeated, tag="2")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
}
/// ClientUpdateProposal is a governance proposal. If it passes, the substitute
/// client's latest consensus state is copied over to the subject client. The proposal
/// handler may fail if the subject and the substitute do not match in client and
/// chain parameters (with exception to latest height, frozen height, and chain-id).
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateProposal {
    /// the title of the update proposal
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag="3")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client standing in for the subject
    /// client
    #[prost(string, tag="4")]
    pub substitute_client_id: ::prost::alloc::string::String,
}
/// UpgradeProposal is a gov Content type for initiating an IBC breaking
/// upgrade.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub plan: ::core::option::Option<super::super::super::super::cosmos::upgrade::v1beta1::Plan>,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades
    #[prost(message, optional, tag="4")]
    pub upgraded_client_state: ::core::option::Option<::prost_wkt_types::Any>,
}
/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the RevisionHeight is incremented at each height while keeping
/// RevisionNumber the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the RevisionNumber is incremented so that
/// height continues to be monitonically increasing even as the RevisionHeight
/// gets reset
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag="1")]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag="2")]
    pub revision_height: u64,
}
/// Params defines the set of IBC light client parameters.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// allowed_clients defines the list of allowed client state types.
    #[prost(string, repeated, tag="1")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCreateClient defines a message to create an IBC client
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClient {
    /// light client state
    #[prost(message, optional, tag="1")]
    pub client_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// consensus state associated with the client that corresponds to a given
    /// height.
    #[prost(message, optional, tag="2")]
    pub consensus_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// signer address
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgCreateClientResponse defines the Msg/CreateClient response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClientResponse {
}
/// MsgUpdateClient defines an sdk.Msg to update a IBC client state using
/// the given header.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClient {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// header to update the light client
    #[prost(message, optional, tag="2")]
    pub header: ::core::option::Option<::prost_wkt_types::Any>,
    /// signer address
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpdateClientResponse defines the Msg/UpdateClient response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClientResponse {
}
/// MsgUpgradeClient defines an sdk.Msg to upgrade an IBC client to a new client
/// state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeClient {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// upgraded client state
    #[prost(message, optional, tag="2")]
    pub client_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// upgraded consensus state, only contains enough information to serve as a
    /// basis of trust in update logic
    #[prost(message, optional, tag="3")]
    pub consensus_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// proof that old chain committed to new client
    #[prost(bytes="vec", tag="4")]
    pub proof_upgrade_client: ::prost::alloc::vec::Vec<u8>,
    /// proof that old chain committed to new consensus state
    #[prost(bytes="vec", tag="5")]
    pub proof_upgrade_consensus_state: ::prost::alloc::vec::Vec<u8>,
    /// signer address
    #[prost(string, tag="6")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgUpgradeClientResponse defines the Msg/UpgradeClient response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpgradeClientResponse {
}
/// MsgSubmitMisbehaviour defines an sdk.Msg type that submits Evidence for
/// light client misbehaviour.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitMisbehaviour {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// misbehaviour used for freezing the light client
    #[prost(message, optional, tag="2")]
    pub misbehaviour: ::core::option::Option<::prost_wkt_types::Any>,
    /// signer address
    #[prost(string, tag="3")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSubmitMisbehaviourResponse defines the Msg/SubmitMisbehaviour response
/// type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitMisbehaviourResponse {
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Msg defines the ibc/client Msg service.
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
        /// CreateClient defines a rpc handler method for MsgCreateClient.
        pub async fn create_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateClient>,
        ) -> Result<tonic::Response<super::MsgCreateClientResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Msg/CreateClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateClient defines a rpc handler method for MsgUpdateClient.
        pub async fn update_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateClient>,
        ) -> Result<tonic::Response<super::MsgUpdateClientResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Msg/UpdateClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpgradeClient defines a rpc handler method for MsgUpgradeClient.
        pub async fn upgrade_client(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpgradeClient>,
        ) -> Result<tonic::Response<super::MsgUpgradeClientResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Msg/UpgradeClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SubmitMisbehaviour defines a rpc handler method for MsgSubmitMisbehaviour.
        pub async fn submit_misbehaviour(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitMisbehaviour>,
        ) -> Result<
                tonic::Response<super::MsgSubmitMisbehaviourResponse>,
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
                "/ibc.core.client.v1.Msg/SubmitMisbehaviour",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// QueryClientStateRequest is the request type for the Query/ClientState RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStateRequest {
    /// client state unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientStateResponse is the response type for the Query/ClientState RPC
/// method. Besides the client state, it includes a proof and the height from
/// which the proof was retrieved.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag="1")]
    pub client_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<Height>,
}
/// QueryClientStatesRequest is the request type for the Query/ClientStates RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatesRequest {
    /// pagination request
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryClientStatesResponse is the response type for the Query/ClientStates RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatesResponse {
    /// list of stored ClientStates of the chain.
    #[prost(message, repeated, tag="1")]
    pub client_states: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryConsensusStateRequest is the request type for the Query/ConsensusState
/// RPC method. Besides the consensus state, it includes a proof and the height
/// from which the proof was retrieved.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateRequest {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus state revision number
    #[prost(uint64, tag="2")]
    pub revision_number: u64,
    /// consensus state revision height
    #[prost(uint64, tag="3")]
    pub revision_height: u64,
    /// latest_height overrrides the height field and queries the latest stored
    /// ConsensusState
    #[prost(bool, tag="4")]
    pub latest_height: bool,
}
/// QueryConsensusStateResponse is the response type for the Query/ConsensusState
/// RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStateResponse {
    /// consensus state associated with the client identifier at the given height
    #[prost(message, optional, tag="1")]
    pub consensus_state: ::core::option::Option<::prost_wkt_types::Any>,
    /// merkle proof of existence
    #[prost(bytes="vec", tag="2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag="3")]
    pub proof_height: ::core::option::Option<Height>,
}
/// QueryConsensusStatesRequest is the request type for the Query/ConsensusStates
/// RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStatesRequest {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryConsensusStatesResponse is the response type for the
/// Query/ConsensusStates RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryConsensusStatesResponse {
    /// consensus states associated with the identifier
    #[prost(message, repeated, tag="1")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
    /// pagination response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClientStatusRequest is the request type for the Query/ClientStatus RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatusRequest {
    /// client unique identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryClientStatusResponse is the response type for the Query/ClientStatus RPC
/// method. It returns the current status of the IBC client.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientStatusResponse {
    #[prost(string, tag="1")]
    pub status: ::prost::alloc::string::String,
}
/// QueryClientParamsRequest is the request type for the Query/ClientParams RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientParamsRequest {
}
/// QueryClientParamsResponse is the response type for the Query/ClientParams RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClientParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryUpgradedClientStateRequest is the request type for the
/// Query/UpgradedClientState RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedClientStateRequest {
}
/// QueryUpgradedClientStateResponse is the response type for the
/// Query/UpgradedClientState RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedClientStateResponse {
    /// client state associated with the request identifier
    #[prost(message, optional, tag="1")]
    pub upgraded_client_state: ::core::option::Option<::prost_wkt_types::Any>,
}
/// QueryUpgradedConsensusStateRequest is the request type for the
/// Query/UpgradedConsensusState RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedConsensusStateRequest {
}
/// QueryUpgradedConsensusStateResponse is the response type for the
/// Query/UpgradedConsensusState RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedConsensusStateResponse {
    /// Consensus state associated with the request identifier
    #[prost(message, optional, tag="1")]
    pub upgraded_consensus_state: ::core::option::Option<::prost_wkt_types::Any>,
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
        /// ClientState queries an IBC light client.
        pub async fn client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientStateRequest>,
        ) -> Result<tonic::Response<super::QueryClientStateResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Query/ClientState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClientStates queries all the IBC light clients of a chain.
        pub async fn client_states(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientStatesRequest>,
        ) -> Result<tonic::Response<super::QueryClientStatesResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Query/ClientStates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ConsensusState queries a consensus state associated with a client state at
        /// a given height.
        pub async fn consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConsensusStateRequest>,
        ) -> Result<tonic::Response<super::QueryConsensusStateResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Query/ConsensusState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ConsensusStates queries all the consensus state associated with a given
        /// client.
        pub async fn consensus_states(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryConsensusStatesRequest>,
        ) -> Result<
                tonic::Response<super::QueryConsensusStatesResponse>,
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
                "/ibc.core.client.v1.Query/ConsensusStates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Status queries the status of an IBC client.
        pub async fn client_status(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientStatusRequest>,
        ) -> Result<tonic::Response<super::QueryClientStatusResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Query/ClientStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClientParams queries all parameters of the ibc client.
        pub async fn client_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClientParamsRequest>,
        ) -> Result<tonic::Response<super::QueryClientParamsResponse>, tonic::Status> {
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
                "/ibc.core.client.v1.Query/ClientParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpgradedClientState queries an Upgraded IBC light client.
        pub async fn upgraded_client_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradedClientStateRequest>,
        ) -> Result<
                tonic::Response<super::QueryUpgradedClientStateResponse>,
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
                "/ibc.core.client.v1.Query/UpgradedClientState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpgradedConsensusState queries an Upgraded IBC consensus state.
        pub async fn upgraded_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradedConsensusStateRequest>,
        ) -> Result<
                tonic::Response<super::QueryUpgradedConsensusStateResponse>,
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
                "/ibc.core.client.v1.Query/UpgradedConsensusState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the ibc client submodule's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// client states with their corresponding identifiers
    #[prost(message, repeated, tag="1")]
    pub clients: ::prost::alloc::vec::Vec<IdentifiedClientState>,
    /// consensus states from each client
    #[prost(message, repeated, tag="2")]
    pub clients_consensus: ::prost::alloc::vec::Vec<ClientConsensusStates>,
    /// metadata from each client
    #[prost(message, repeated, tag="3")]
    pub clients_metadata: ::prost::alloc::vec::Vec<IdentifiedGenesisMetadata>,
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<Params>,
    /// create localhost on initialization
    #[prost(bool, tag="5")]
    pub create_localhost: bool,
    /// the sequence for the next generated client identifier
    #[prost(uint64, tag="6")]
    pub next_client_sequence: u64,
}
/// GenesisMetadata defines the genesis type for metadata that clients may return
/// with ExportMetadata
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisMetadata {
    /// store key of metadata without clientID-prefix
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// metadata value
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// IdentifiedGenesisMetadata has the client metadata with the corresponding
/// client id.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedGenesisMetadata {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub client_metadata: ::prost::alloc::vec::Vec<GenesisMetadata>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_IDENTIFIED_CLIENT_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.IdentifiedClientState")] impl :: prost_wkt :: MessageSerde for IdentifiedClientState { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "IdentifiedClientState" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.IdentifiedClientState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CONSENSUS_STATE_WITH_HEIGHT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.ConsensusStateWithHeight")] impl :: prost_wkt :: MessageSerde for ConsensusStateWithHeight { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "ConsensusStateWithHeight" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.ConsensusStateWithHeight" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CLIENT_CONSENSUS_STATES : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.ClientConsensusStates")] impl :: prost_wkt :: MessageSerde for ClientConsensusStates { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "ClientConsensusStates" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.ClientConsensusStates" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CLIENT_UPDATE_PROPOSAL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.ClientUpdateProposal")] impl :: prost_wkt :: MessageSerde for ClientUpdateProposal { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "ClientUpdateProposal" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.ClientUpdateProposal" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPGRADE_PROPOSAL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.UpgradeProposal")] impl :: prost_wkt :: MessageSerde for UpgradeProposal { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "UpgradeProposal" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.UpgradeProposal" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_HEIGHT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.Height")] impl :: prost_wkt :: MessageSerde for Height { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "Height" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.Height" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_CLIENT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgCreateClient")] impl :: prost_wkt :: MessageSerde for MsgCreateClient { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgCreateClient" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgCreateClient" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_CLIENT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgCreateClientResponse")] impl :: prost_wkt :: MessageSerde for MsgCreateClientResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgCreateClientResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgCreateClientResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_UPDATE_CLIENT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgUpdateClient")] impl :: prost_wkt :: MessageSerde for MsgUpdateClient { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgUpdateClient" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgUpdateClient" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_UPDATE_CLIENT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgUpdateClientResponse")] impl :: prost_wkt :: MessageSerde for MsgUpdateClientResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgUpdateClientResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgUpdateClientResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_UPGRADE_CLIENT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgUpgradeClient")] impl :: prost_wkt :: MessageSerde for MsgUpgradeClient { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgUpgradeClient" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgUpgradeClient" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_UPGRADE_CLIENT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgUpgradeClientResponse")] impl :: prost_wkt :: MessageSerde for MsgUpgradeClientResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgUpgradeClientResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgUpgradeClientResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUBMIT_MISBEHAVIOUR : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgSubmitMisbehaviour")] impl :: prost_wkt :: MessageSerde for MsgSubmitMisbehaviour { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgSubmitMisbehaviour" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgSubmitMisbehaviour" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUBMIT_MISBEHAVIOUR_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.MsgSubmitMisbehaviourResponse")] impl :: prost_wkt :: MessageSerde for MsgSubmitMisbehaviourResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "MsgSubmitMisbehaviourResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.MsgSubmitMisbehaviourResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_STATE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientStateRequest")] impl :: prost_wkt :: MessageSerde for QueryClientStateRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientStateRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientStateRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_STATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientStateResponse")] impl :: prost_wkt :: MessageSerde for QueryClientStateResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientStateResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientStateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_STATES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientStatesRequest")] impl :: prost_wkt :: MessageSerde for QueryClientStatesRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientStatesRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientStatesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_STATES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientStatesResponse")] impl :: prost_wkt :: MessageSerde for QueryClientStatesResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientStatesResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientStatesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONSENSUS_STATE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryConsensusStateRequest")] impl :: prost_wkt :: MessageSerde for QueryConsensusStateRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryConsensusStateRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryConsensusStateRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONSENSUS_STATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryConsensusStateResponse")] impl :: prost_wkt :: MessageSerde for QueryConsensusStateResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryConsensusStateResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryConsensusStateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONSENSUS_STATES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryConsensusStatesRequest")] impl :: prost_wkt :: MessageSerde for QueryConsensusStatesRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryConsensusStatesRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryConsensusStatesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONSENSUS_STATES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryConsensusStatesResponse")] impl :: prost_wkt :: MessageSerde for QueryConsensusStatesResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryConsensusStatesResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryConsensusStatesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_STATUS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientStatusRequest")] impl :: prost_wkt :: MessageSerde for QueryClientStatusRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientStatusRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientStatusRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_STATUS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientStatusResponse")] impl :: prost_wkt :: MessageSerde for QueryClientStatusResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientStatusResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientStatusResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryClientParamsRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientParamsRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CLIENT_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryClientParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryClientParamsResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryClientParamsResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryClientParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UPGRADED_CLIENT_STATE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryUpgradedClientStateRequest")] impl :: prost_wkt :: MessageSerde for QueryUpgradedClientStateRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryUpgradedClientStateRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryUpgradedClientStateRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UPGRADED_CLIENT_STATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryUpgradedClientStateResponse")] impl :: prost_wkt :: MessageSerde for QueryUpgradedClientStateResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryUpgradedClientStateResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryUpgradedClientStateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UPGRADED_CONSENSUS_STATE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryUpgradedConsensusStateRequest")] impl :: prost_wkt :: MessageSerde for QueryUpgradedConsensusStateRequest { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryUpgradedConsensusStateRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryUpgradedConsensusStateRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_UPGRADED_CONSENSUS_STATE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.QueryUpgradedConsensusStateResponse")] impl :: prost_wkt :: MessageSerde for QueryUpgradedConsensusStateResponse { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "QueryUpgradedConsensusStateResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.QueryUpgradedConsensusStateResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_METADATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.GenesisMetadata")] impl :: prost_wkt :: MessageSerde for GenesisMetadata { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "GenesisMetadata" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.GenesisMetadata" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_IDENTIFIED_GENESIS_METADATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.client.v1.IdentifiedGenesisMetadata")] impl :: prost_wkt :: MessageSerde for IdentifiedGenesisMetadata { fn package_name (& self) -> & 'static str { "ibc.core.client.v1" } fn message_name (& self) -> & 'static str { "IdentifiedGenesisMetadata" } fn type_url (& self) -> & 'static str { "/ibc.core.client.v1.IdentifiedGenesisMetadata" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
