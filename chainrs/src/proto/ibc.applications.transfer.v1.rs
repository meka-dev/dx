/// DenomTrace contains the base denomination for ICS20 fungible tokens and the
/// source tracing information path.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomTrace {
    /// path defines the chain of port/channel identifiers used for tracing the
    /// source of the fungible token.
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// base denomination of the relayed fungible token.
    #[prost(string, tag="2")]
    pub base_denom: ::prost::alloc::string::String,
}
/// Params defines the set of IBC transfer parameters.
/// NOTE: To prevent a single token from being transferred, set the
/// TransfersEnabled parameter to true and then set the bank module's SendEnabled
/// parameter for the denomination to false.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// send_enabled enables or disables all cross-chain token transfers from this
    /// chain.
    #[prost(bool, tag="1")]
    pub send_enabled: bool,
    /// receive_enabled enables or disables all cross-chain token transfers to this
    /// chain.
    #[prost(bool, tag="2")]
    pub receive_enabled: bool,
}
/// GenesisState defines the ibc-transfer genesis state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub denom_traces: ::prost::alloc::vec::Vec<DenomTrace>,
    #[prost(message, optional, tag="3")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDenomTraceRequest is the request type for the Query/DenomTrace RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTraceRequest {
    /// hash (in hex format) of the denomination trace information.
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryDenomTraceResponse is the response type for the Query/DenomTrace RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTraceResponse {
    /// denom_trace returns the requested denomination trace information.
    #[prost(message, optional, tag="1")]
    pub denom_trace: ::core::option::Option<DenomTrace>,
}
/// QueryConnectionsRequest is the request type for the Query/DenomTraces RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTracesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryConnectionsResponse is the response type for the Query/DenomTraces RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomTracesResponse {
    /// denom_traces returns all denominations trace information.
    #[prost(message, repeated, tag="1")]
    pub denom_traces: ::prost::alloc::vec::Vec<DenomTrace>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between
/// ICS20 enabled chains. See ICS Spec here:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag="1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag="2")]
    pub source_channel: ::prost::alloc::string::String,
    /// the tokens to be transferred
    #[prost(message, optional, tag="3")]
    pub token: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the sender address
    #[prost(string, tag="4")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag="5")]
    pub receiver: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag="6")]
    pub timeout_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Timeout timestamp (in nanoseconds) relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag="7")]
    pub timeout_timestamp: u64,
}
/// MsgTransferResponse defines the Msg/Transfer response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferResponse {
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_DENOM_TRACE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.DenomTrace")] impl :: prost_wkt :: MessageSerde for DenomTrace { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "DenomTrace" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.DenomTrace" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_DENOM_TRACE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.QueryDenomTraceRequest")] impl :: prost_wkt :: MessageSerde for QueryDenomTraceRequest { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "QueryDenomTraceRequest" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.QueryDenomTraceRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_DENOM_TRACE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.QueryDenomTraceResponse")] impl :: prost_wkt :: MessageSerde for QueryDenomTraceResponse { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "QueryDenomTraceResponse" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.QueryDenomTraceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_DENOM_TRACES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.QueryDenomTracesRequest")] impl :: prost_wkt :: MessageSerde for QueryDenomTracesRequest { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "QueryDenomTracesRequest" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.QueryDenomTracesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_DENOM_TRACES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.QueryDenomTracesResponse")] impl :: prost_wkt :: MessageSerde for QueryDenomTracesResponse { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "QueryDenomTracesResponse" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.QueryDenomTracesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.QueryParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryParamsRequest { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "QueryParamsRequest" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.QueryParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.QueryParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryParamsResponse { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "QueryParamsResponse" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.QueryParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_TRANSFER : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.MsgTransfer")] impl :: prost_wkt :: MessageSerde for MsgTransfer { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "MsgTransfer" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.MsgTransfer" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_TRANSFER_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v1.MsgTransferResponse")] impl :: prost_wkt :: MessageSerde for MsgTransferResponse { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v1" } fn message_name (& self) -> & 'static str { "MsgTransferResponse" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v1.MsgTransferResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
