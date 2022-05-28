/// QueryAppVersionRequest is the request type for the Query/AppVersion RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppVersionRequest {
    /// port unique identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// connection unique identifier
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// whether the channel is ordered or unordered
    #[prost(enumeration="super::super::channel::v1::Order", tag="3")]
    pub ordering: i32,
    /// counterparty channel end
    #[prost(message, optional, tag="4")]
    pub counterparty: ::core::option::Option<super::super::channel::v1::Counterparty>,
    /// proposed version
    #[prost(string, tag="5")]
    pub proposed_version: ::prost::alloc::string::String,
}
/// QueryAppVersionResponse is the response type for the Query/AppVersion RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppVersionResponse {
    /// port id associated with the request identifiers
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// supported app version
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Query defines the gRPC querier service
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
        /// AppVersion queries an IBC Port and determines the appropriate application version to be used
        pub async fn app_version(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAppVersionRequest>,
        ) -> Result<tonic::Response<super::QueryAppVersionResponse>, tonic::Status> {
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
                "/ibc.core.port.v1.Query/AppVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_APP_VERSION_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.port.v1.QueryAppVersionRequest")] impl :: prost_wkt :: MessageSerde for QueryAppVersionRequest { fn package_name (& self) -> & 'static str { "ibc.core.port.v1" } fn message_name (& self) -> & 'static str { "QueryAppVersionRequest" } fn type_url (& self) -> & 'static str { "/ibc.core.port.v1.QueryAppVersionRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_APP_VERSION_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.core.port.v1.QueryAppVersionResponse")] impl :: prost_wkt :: MessageSerde for QueryAppVersionResponse { fn package_name (& self) -> & 'static str { "ibc.core.port.v1" } fn message_name (& self) -> & 'static str { "QueryAppVersionResponse" } fn type_url (& self) -> & 'static str { "/ibc.core.port.v1.QueryAppVersionResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
