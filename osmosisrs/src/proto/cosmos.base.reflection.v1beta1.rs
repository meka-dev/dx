/// ListAllInterfacesRequest is the request type of the ListAllInterfaces RPC.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterfacesRequest {
}
/// ListAllInterfacesResponse is the response type of the ListAllInterfaces RPC.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterfacesResponse {
    /// interface_names is an array of all the registered interfaces.
    #[prost(string, repeated, tag="1")]
    pub interface_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ListImplementationsRequest is the request type of the ListImplementations
/// RPC.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImplementationsRequest {
    /// interface_name defines the interface to query the implementations for.
    #[prost(string, tag="1")]
    pub interface_name: ::prost::alloc::string::String,
}
/// ListImplementationsResponse is the response type of the ListImplementations
/// RPC.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImplementationsResponse {
    #[prost(string, repeated, tag="1")]
    pub implementation_message_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod reflection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// ReflectionService defines a service for interface reflection.
    #[derive(Debug, Clone)]
    pub struct ReflectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReflectionServiceClient<T>
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
        ) -> ReflectionServiceClient<InterceptedService<T, F>>
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
            ReflectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// ListAllInterfaces lists all the interfaces registered in the interface
        /// registry.
        pub async fn list_all_interfaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAllInterfacesRequest>,
        ) -> Result<tonic::Response<super::ListAllInterfacesResponse>, tonic::Status> {
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
                "/cosmos.base.reflection.v1beta1.ReflectionService/ListAllInterfaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ListImplementations list all the concrete types that implement a given
        /// interface.
        pub async fn list_implementations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImplementationsRequest>,
        ) -> Result<tonic::Response<super::ListImplementationsResponse>, tonic::Status> {
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
                "/cosmos.base.reflection.v1beta1.ReflectionService/ListImplementations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_ALL_INTERFACES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/cosmos.base.reflection.v1beta1.ListAllInterfacesRequest")] impl :: prost_wkt :: MessageSerde for ListAllInterfacesRequest { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListAllInterfacesRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/cosmos.base.reflection.v1beta1.ListAllInterfacesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_ALL_INTERFACES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/cosmos.base.reflection.v1beta1.ListAllInterfacesResponse")] impl :: prost_wkt :: MessageSerde for ListAllInterfacesResponse { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListAllInterfacesResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/cosmos.base.reflection.v1beta1.ListAllInterfacesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_IMPLEMENTATIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/cosmos.base.reflection.v1beta1.ListImplementationsRequest")] impl :: prost_wkt :: MessageSerde for ListImplementationsRequest { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListImplementationsRequest" } fn type_url (& self) -> & 'static str { "type.googleapis.com/cosmos.base.reflection.v1beta1.ListImplementationsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_IMPLEMENTATIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/cosmos.base.reflection.v1beta1.ListImplementationsResponse")] impl :: prost_wkt :: MessageSerde for ListImplementationsResponse { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListImplementationsResponse" } fn type_url (& self) -> & 'static str { "type.googleapis.com/cosmos.base.reflection.v1beta1.ListImplementationsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
