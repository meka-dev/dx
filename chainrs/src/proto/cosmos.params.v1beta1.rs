/// ParameterChangeProposal defines a proposal to change one or more parameters.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterChangeProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub changes: ::prost::alloc::vec::Vec<ParamChange>,
}
/// ParamChange defines an individual parameter change, for use in
/// ParameterChangeProposal.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamChange {
    #[prost(string, tag="1")]
    pub subspace: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub value: ::prost::alloc::string::String,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
    /// subspace defines the module to query the parameter for.
    #[prost(string, tag="1")]
    pub subspace: ::prost::alloc::string::String,
    /// key defines the key of the parameter in the subspace.
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// param defines the queried parameter.
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<ParamChange>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMETER_CHANGE_PROPOSAL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.params.v1beta1.ParameterChangeProposal")] impl :: prost_wkt :: MessageSerde for ParameterChangeProposal { fn package_name (& self) -> & 'static str { "cosmos.params.v1beta1" } fn message_name (& self) -> & 'static str { "ParameterChangeProposal" } fn type_url (& self) -> & 'static str { "/cosmos.params.v1beta1.ParameterChangeProposal" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAM_CHANGE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.params.v1beta1.ParamChange")] impl :: prost_wkt :: MessageSerde for ParamChange { fn package_name (& self) -> & 'static str { "cosmos.params.v1beta1" } fn message_name (& self) -> & 'static str { "ParamChange" } fn type_url (& self) -> & 'static str { "/cosmos.params.v1beta1.ParamChange" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.params.v1beta1.QueryParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryParamsRequest { fn package_name (& self) -> & 'static str { "cosmos.params.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsRequest" } fn type_url (& self) -> & 'static str { "/cosmos.params.v1beta1.QueryParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.params.v1beta1.QueryParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryParamsResponse { fn package_name (& self) -> & 'static str { "cosmos.params.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsResponse" } fn type_url (& self) -> & 'static str { "/cosmos.params.v1beta1.QueryParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
