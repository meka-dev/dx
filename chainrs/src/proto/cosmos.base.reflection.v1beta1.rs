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

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_ALL_INTERFACES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.reflection.v1beta1.ListAllInterfacesRequest")] impl :: prost_wkt :: MessageSerde for ListAllInterfacesRequest { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListAllInterfacesRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.reflection.v1beta1.ListAllInterfacesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_ALL_INTERFACES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.reflection.v1beta1.ListAllInterfacesResponse")] impl :: prost_wkt :: MessageSerde for ListAllInterfacesResponse { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListAllInterfacesResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.reflection.v1beta1.ListAllInterfacesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_IMPLEMENTATIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.reflection.v1beta1.ListImplementationsRequest")] impl :: prost_wkt :: MessageSerde for ListImplementationsRequest { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListImplementationsRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.reflection.v1beta1.ListImplementationsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LIST_IMPLEMENTATIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.reflection.v1beta1.ListImplementationsResponse")] impl :: prost_wkt :: MessageSerde for ListImplementationsResponse { fn package_name (& self) -> & 'static str { "cosmos.base.reflection.v1beta1" } fn message_name (& self) -> & 'static str { "ListImplementationsResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.reflection.v1beta1.ListImplementationsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
