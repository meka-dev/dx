/// GetValidatorSetByHeightRequest is the request type for the Query/GetValidatorSetByHeight RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetByHeightRequest {
    #[prost(int64, tag="1")]
    pub height: i64,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetByHeightResponse {
    #[prost(int64, tag="1")]
    pub block_height: i64,
    #[prost(message, repeated, tag="2")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="3")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// GetLatestValidatorSetRequest is the request type for the Query/GetValidatorSetByHeight RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestValidatorSetRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetLatestValidatorSetResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestValidatorSetResponse {
    #[prost(int64, tag="1")]
    pub block_height: i64,
    #[prost(message, repeated, tag="2")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="3")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// Validator is the type for the validator-set.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pub_key: ::core::option::Option<::prost_wkt_types::Any>,
    #[prost(int64, tag="3")]
    pub voting_power: i64,
    #[prost(int64, tag="4")]
    pub proposer_priority: i64,
}
/// GetBlockByHeightRequest is the request type for the Query/GetBlockByHeight RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightRequest {
    #[prost(int64, tag="1")]
    pub height: i64,
}
/// GetBlockByHeightResponse is the response type for the Query/GetBlockByHeight RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightResponse {
    #[prost(message, optional, tag="1")]
    pub block_id: ::core::option::Option<::tendermint_proto::types::BlockId>,
    #[prost(message, optional, tag="2")]
    pub block: ::core::option::Option<::tendermint_proto::types::Block>,
}
/// GetLatestBlockRequest is the request type for the Query/GetLatestBlock RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockRequest {
}
/// GetLatestBlockResponse is the response type for the Query/GetLatestBlock RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockResponse {
    #[prost(message, optional, tag="1")]
    pub block_id: ::core::option::Option<::tendermint_proto::types::BlockId>,
    #[prost(message, optional, tag="2")]
    pub block: ::core::option::Option<::tendermint_proto::types::Block>,
}
/// GetSyncingRequest is the request type for the Query/GetSyncing RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncingRequest {
}
/// GetSyncingResponse is the response type for the Query/GetSyncing RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncingResponse {
    #[prost(bool, tag="1")]
    pub syncing: bool,
}
/// GetNodeInfoRequest is the request type for the Query/GetNodeInfo RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoRequest {
}
/// GetNodeInfoResponse is the request type for the Query/GetNodeInfo RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoResponse {
    #[prost(message, optional, tag="1")]
    #[serde(skip)]
    pub default_node_info: ::core::option::Option<::tendermint_proto::p2p::DefaultNodeInfo>,
    #[prost(message, optional, tag="2")]
    pub application_version: ::core::option::Option<VersionInfo>,
}
/// VersionInfo is the type for the GetNodeInfoResponse message.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub app_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub git_commit: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub build_tags: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub go_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="7")]
    pub build_deps: ::prost::alloc::vec::Vec<Module>,
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag="8")]
    pub cosmos_sdk_version: ::prost::alloc::string::String,
}
/// Module is the type for VersionInfo
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// module path
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// module version
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// checksum
    #[prost(string, tag="3")]
    pub sum: ::prost::alloc::string::String,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_VALIDATOR_SET_BY_HEIGHT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightRequest")] impl :: prost_wkt :: MessageSerde for GetValidatorSetByHeightRequest { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetValidatorSetByHeightRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_VALIDATOR_SET_BY_HEIGHT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightResponse")] impl :: prost_wkt :: MessageSerde for GetValidatorSetByHeightResponse { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetValidatorSetByHeightResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetValidatorSetByHeightResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_LATEST_VALIDATOR_SET_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetLatestValidatorSetRequest")] impl :: prost_wkt :: MessageSerde for GetLatestValidatorSetRequest { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetLatestValidatorSetRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetLatestValidatorSetRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_LATEST_VALIDATOR_SET_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetLatestValidatorSetResponse")] impl :: prost_wkt :: MessageSerde for GetLatestValidatorSetResponse { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetLatestValidatorSetResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetLatestValidatorSetResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_VALIDATOR : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.Validator")] impl :: prost_wkt :: MessageSerde for Validator { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "Validator" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.Validator" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_BLOCK_BY_HEIGHT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetBlockByHeightRequest")] impl :: prost_wkt :: MessageSerde for GetBlockByHeightRequest { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetBlockByHeightRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetBlockByHeightRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_BLOCK_BY_HEIGHT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetBlockByHeightResponse")] impl :: prost_wkt :: MessageSerde for GetBlockByHeightResponse { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetBlockByHeightResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetBlockByHeightResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_LATEST_BLOCK_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetLatestBlockRequest")] impl :: prost_wkt :: MessageSerde for GetLatestBlockRequest { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetLatestBlockRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetLatestBlockRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_LATEST_BLOCK_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetLatestBlockResponse")] impl :: prost_wkt :: MessageSerde for GetLatestBlockResponse { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetLatestBlockResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetLatestBlockResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_SYNCING_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetSyncingRequest")] impl :: prost_wkt :: MessageSerde for GetSyncingRequest { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetSyncingRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetSyncingRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_SYNCING_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetSyncingResponse")] impl :: prost_wkt :: MessageSerde for GetSyncingResponse { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetSyncingResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetSyncingResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_NODE_INFO_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetNodeInfoRequest")] impl :: prost_wkt :: MessageSerde for GetNodeInfoRequest { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetNodeInfoRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetNodeInfoRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GET_NODE_INFO_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.GetNodeInfoResponse")] impl :: prost_wkt :: MessageSerde for GetNodeInfoResponse { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "GetNodeInfoResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.GetNodeInfoResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_VERSION_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.VersionInfo")] impl :: prost_wkt :: MessageSerde for VersionInfo { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "VersionInfo" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.VersionInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.tendermint.v1beta1.Module")] impl :: prost_wkt :: MessageSerde for Module { fn package_name (& self) -> & 'static str { "cosmos.base.tendermint.v1beta1" } fn message_name (& self) -> & 'static str { "Module" } fn type_url (& self) -> & 'static str { "/cosmos.base.tendermint.v1beta1.Module" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
