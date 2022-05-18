/// ValidatorSigningInfo defines a validator's signing info for monitoring their
/// liveness activity.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSigningInfo {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// Height at which validator was first a candidate OR was unjailed
    #[prost(int64, tag="2")]
    pub start_height: i64,
    /// Index which is incremented each time the validator was a bonded
    /// in a block and may have signed a precommit or not. This in conjunction with the
    /// `SignedBlocksWindow` param determines the index in the `MissedBlocksBitArray`.
    #[prost(int64, tag="3")]
    pub index_offset: i64,
    /// Timestamp until which the validator is jailed due to liveness downtime.
    #[prost(message, optional, tag="4")]
    pub jailed_until: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// Whether or not a validator has been tombstoned (killed out of validator set). It is set
    /// once the validator commits an equivocation or for any other configured misbehiavor.
    #[prost(bool, tag="5")]
    pub tombstoned: bool,
    /// A counter kept to avoid unnecessary array reads.
    /// Note that `Sum(MissedBlocksBitArray)` always equals `MissedBlocksCounter`.
    #[prost(int64, tag="6")]
    pub missed_blocks_counter: i64,
}
/// Params represents the parameters used for by the slashing module.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(int64, tag="1")]
    pub signed_blocks_window: i64,
    #[prost(bytes="vec", tag="2")]
    pub min_signed_per_window: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub downtime_jail_duration: ::core::option::Option<::prost_wkt_types::Duration>,
    #[prost(bytes="vec", tag="4")]
    pub slash_fraction_double_sign: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub slash_fraction_downtime: ::prost::alloc::vec::Vec<u8>,
}
/// GenesisState defines the slashing module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of related to deposit.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// signing_infos represents a map between validator addresses and their
    /// signing infos.
    #[prost(message, repeated, tag="2")]
    pub signing_infos: ::prost::alloc::vec::Vec<SigningInfo>,
    /// missed_blocks represents a map between validator addresses and their
    /// missed blocks.
    #[prost(message, repeated, tag="3")]
    pub missed_blocks: ::prost::alloc::vec::Vec<ValidatorMissedBlocks>,
}
/// SigningInfo stores validator signing info of corresponding address.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningInfo {
    /// address is the validator address.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// validator_signing_info represents the signing info of this validator.
    #[prost(message, optional, tag="2")]
    pub validator_signing_info: ::core::option::Option<ValidatorSigningInfo>,
}
/// ValidatorMissedBlocks contains array of missed blocks of corresponding
/// address.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorMissedBlocks {
    /// address is the validator address.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// missed_blocks is an array of missed blocks by the validator.
    #[prost(message, repeated, tag="2")]
    pub missed_blocks: ::prost::alloc::vec::Vec<MissedBlock>,
}
/// MissedBlock contains height and missed status as boolean.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissedBlock {
    /// index is the height at which the block was missed.
    #[prost(int64, tag="1")]
    pub index: i64,
    /// missed is the missed status.
    #[prost(bool, tag="2")]
    pub missed: bool,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QuerySigningInfoRequest is the request type for the Query/SigningInfo RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfoRequest {
    /// cons_address is the address to query signing info of
    #[prost(string, tag="1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QuerySigningInfoResponse is the response type for the Query/SigningInfo RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfoResponse {
    /// val_signing_info is the signing info of requested val cons address
    #[prost(message, optional, tag="1")]
    pub val_signing_info: ::core::option::Option<ValidatorSigningInfo>,
}
/// QuerySigningInfosRequest is the request type for the Query/SigningInfos RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfosRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QuerySigningInfosResponse is the response type for the Query/SigningInfos RPC
/// method
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningInfosResponse {
    /// info is the signing info of all validators
    #[prost(message, repeated, tag="1")]
    pub info: ::prost::alloc::vec::Vec<ValidatorSigningInfo>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgUnjail defines the Msg/Unjail request type
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjail {
    #[prost(string, tag="1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// MsgUnjailResponse defines the Msg/Unjail response type
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnjailResponse {
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_SIGNING_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.ValidatorSigningInfo")] impl :: prost_wkt :: MessageSerde for ValidatorSigningInfo { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "ValidatorSigningInfo" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.ValidatorSigningInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SIGNING_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.SigningInfo")] impl :: prost_wkt :: MessageSerde for SigningInfo { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "SigningInfo" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.SigningInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_VALIDATOR_MISSED_BLOCKS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.ValidatorMissedBlocks")] impl :: prost_wkt :: MessageSerde for ValidatorMissedBlocks { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "ValidatorMissedBlocks" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.ValidatorMissedBlocks" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MISSED_BLOCK : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.MissedBlock")] impl :: prost_wkt :: MessageSerde for MissedBlock { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "MissedBlock" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.MissedBlock" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.QueryParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryParamsRequest { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsRequest" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.QueryParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.QueryParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryParamsResponse { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsResponse" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.QueryParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_SIGNING_INFO_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.QuerySigningInfoRequest")] impl :: prost_wkt :: MessageSerde for QuerySigningInfoRequest { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "QuerySigningInfoRequest" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.QuerySigningInfoRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_SIGNING_INFO_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.QuerySigningInfoResponse")] impl :: prost_wkt :: MessageSerde for QuerySigningInfoResponse { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "QuerySigningInfoResponse" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.QuerySigningInfoResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_SIGNING_INFOS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.QuerySigningInfosRequest")] impl :: prost_wkt :: MessageSerde for QuerySigningInfosRequest { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "QuerySigningInfosRequest" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.QuerySigningInfosRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_SIGNING_INFOS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.QuerySigningInfosResponse")] impl :: prost_wkt :: MessageSerde for QuerySigningInfosResponse { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "QuerySigningInfosResponse" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.QuerySigningInfosResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_UNJAIL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.MsgUnjail")] impl :: prost_wkt :: MessageSerde for MsgUnjail { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "MsgUnjail" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.MsgUnjail" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_UNJAIL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.slashing.v1beta1.MsgUnjailResponse")] impl :: prost_wkt :: MessageSerde for MsgUnjailResponse { fn package_name (& self) -> & 'static str { "cosmos.slashing.v1beta1" } fn message_name (& self) -> & 'static str { "MsgUnjailResponse" } fn type_url (& self) -> & 'static str { "/cosmos.slashing.v1beta1.MsgUnjailResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
