#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochInfo {
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
    #[prost(int64, tag="4")]
    pub current_epoch: i64,
    #[prost(message, optional, tag="5")]
    pub current_epoch_start_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(bool, tag="6")]
    pub epoch_counting_started: bool,
    #[prost(int64, tag="8")]
    pub current_epoch_start_height: i64,
}
/// GenesisState defines the epochs module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochsInfoResponse {
    #[prost(message, repeated, tag="1")]
    pub epochs: ::prost::alloc::vec::Vec<EpochInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochRequest {
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochResponse {
    #[prost(int64, tag="1")]
    pub current_epoch: i64,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EPOCH_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.epochs.v1beta1.EpochInfo")] impl :: prost_wkt :: MessageSerde for EpochInfo { fn package_name (& self) -> & 'static str { "osmosis.epochs.v1beta1" } fn message_name (& self) -> & 'static str { "EpochInfo" } fn type_url (& self) -> & 'static str { "/osmosis.epochs.v1beta1.EpochInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.epochs.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "osmosis.epochs.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/osmosis.epochs.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_EPOCHS_INFO_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.epochs.v1beta1.QueryEpochsInfoRequest")] impl :: prost_wkt :: MessageSerde for QueryEpochsInfoRequest { fn package_name (& self) -> & 'static str { "osmosis.epochs.v1beta1" } fn message_name (& self) -> & 'static str { "QueryEpochsInfoRequest" } fn type_url (& self) -> & 'static str { "/osmosis.epochs.v1beta1.QueryEpochsInfoRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_EPOCHS_INFO_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.epochs.v1beta1.QueryEpochsInfoResponse")] impl :: prost_wkt :: MessageSerde for QueryEpochsInfoResponse { fn package_name (& self) -> & 'static str { "osmosis.epochs.v1beta1" } fn message_name (& self) -> & 'static str { "QueryEpochsInfoResponse" } fn type_url (& self) -> & 'static str { "/osmosis.epochs.v1beta1.QueryEpochsInfoResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CURRENT_EPOCH_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.epochs.v1beta1.QueryCurrentEpochRequest")] impl :: prost_wkt :: MessageSerde for QueryCurrentEpochRequest { fn package_name (& self) -> & 'static str { "osmosis.epochs.v1beta1" } fn message_name (& self) -> & 'static str { "QueryCurrentEpochRequest" } fn type_url (& self) -> & 'static str { "/osmosis.epochs.v1beta1.QueryCurrentEpochRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CURRENT_EPOCH_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.epochs.v1beta1.QueryCurrentEpochResponse")] impl :: prost_wkt :: MessageSerde for QueryCurrentEpochResponse { fn package_name (& self) -> & 'static str { "osmosis.epochs.v1beta1" } fn message_name (& self) -> & 'static str { "QueryCurrentEpochResponse" } fn type_url (& self) -> & 'static str { "/osmosis.epochs.v1beta1.QueryCurrentEpochResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
