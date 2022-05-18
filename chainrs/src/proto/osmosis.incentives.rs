/// Params holds parameters for the incentives module
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// distribution epoch identifier
    #[prost(string, tag="1")]
    pub distr_epoch_identifier: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gauge {
    /// unique ID of a Gauge
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// flag to show if it's perpetual or multi-epoch
    #[prost(bool, tag="2")]
    pub is_perpetual: bool,
    /// distribution incentives by third party
    /// Rewards are distributed to lockups that are are returned by at least one of
    /// these queries
    #[prost(message, optional, tag="3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// total amount of Coins that has been in the gauge.
    ///
    /// can distribute multiple coins
    #[prost(message, repeated, tag="4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag="5")]
    pub start_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag="6")]
    pub num_epochs_paid_over: u64,
    /// number of epochs distributed already
    #[prost(uint64, tag="7")]
    pub filled_epochs: u64,
    /// already distributed coins
    #[prost(message, repeated, tag="8")]
    pub distributed_coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockableDurationsInfo {
    #[prost(message, repeated, tag="1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_wkt_types::Duration>,
}
/// GenesisState defines the incentives module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, repeated, tag="3")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_wkt_types::Duration>,
    #[prost(uint64, tag="4")]
    pub last_gauge_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleToDistributeCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleDistributedCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugeByIdResponse {
    #[prost(message, optional, tag="1")]
    pub gauge: ::core::option::Option<Gauge>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GaugesResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveGaugesPerDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Gauge>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpcomingGaugesPerDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub upcoming_gauges: ::prost::alloc::vec::Vec<Gauge>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="2")]
    pub lock_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(int64, tag="3")]
    pub end_epoch: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsEstResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLockableDurationsResponse {
    #[prost(message, repeated, tag="1")]
    pub lockable_durations: ::prost::alloc::vec::Vec<::prost_wkt_types::Duration>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGauge {
    /// flag to show if it's perpetual or multi-epoch
    #[prost(bool, tag="1")]
    pub is_perpetual: bool,
    /// distribution incentives by third party
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// distribute condition of a lock which meet one of these conditions
    #[prost(message, optional, tag="3")]
    pub distribute_to: ::core::option::Option<super::lockup::QueryCondition>,
    /// can distribute multiple coins
    #[prost(message, repeated, tag="4")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// distribution start time
    #[prost(message, optional, tag="5")]
    pub start_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// number of epochs distribution will be done
    #[prost(uint64, tag="6")]
    pub num_epochs_paid_over: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateGaugeResponse {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGauge {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub gauge_id: u64,
    #[prost(message, repeated, tag="3")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddToGaugeResponse {
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GAUGE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.Gauge")] impl :: prost_wkt :: MessageSerde for Gauge { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "Gauge" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.Gauge" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LOCKABLE_DURATIONS_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.LockableDurationsInfo")] impl :: prost_wkt :: MessageSerde for LockableDurationsInfo { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "LockableDurationsInfo" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.LockableDurationsInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_TO_DISTRIBUTE_COINS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ModuleToDistributeCoinsRequest")] impl :: prost_wkt :: MessageSerde for ModuleToDistributeCoinsRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ModuleToDistributeCoinsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ModuleToDistributeCoinsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_TO_DISTRIBUTE_COINS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ModuleToDistributeCoinsResponse")] impl :: prost_wkt :: MessageSerde for ModuleToDistributeCoinsResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ModuleToDistributeCoinsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ModuleToDistributeCoinsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_DISTRIBUTED_COINS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ModuleDistributedCoinsRequest")] impl :: prost_wkt :: MessageSerde for ModuleDistributedCoinsRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ModuleDistributedCoinsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ModuleDistributedCoinsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_DISTRIBUTED_COINS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ModuleDistributedCoinsResponse")] impl :: prost_wkt :: MessageSerde for ModuleDistributedCoinsResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ModuleDistributedCoinsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ModuleDistributedCoinsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GAUGE_BY_ID_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.GaugeByIDRequest")] impl :: prost_wkt :: MessageSerde for GaugeByIdRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "GaugeByIDRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.GaugeByIDRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GAUGE_BY_ID_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.GaugeByIDResponse")] impl :: prost_wkt :: MessageSerde for GaugeByIdResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "GaugeByIDResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.GaugeByIDResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GAUGES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.GaugesRequest")] impl :: prost_wkt :: MessageSerde for GaugesRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "GaugesRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.GaugesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GAUGES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.GaugesResponse")] impl :: prost_wkt :: MessageSerde for GaugesResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "GaugesResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.GaugesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACTIVE_GAUGES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ActiveGaugesRequest")] impl :: prost_wkt :: MessageSerde for ActiveGaugesRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ActiveGaugesRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ActiveGaugesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACTIVE_GAUGES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ActiveGaugesResponse")] impl :: prost_wkt :: MessageSerde for ActiveGaugesResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ActiveGaugesResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ActiveGaugesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACTIVE_GAUGES_PER_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ActiveGaugesPerDenomRequest")] impl :: prost_wkt :: MessageSerde for ActiveGaugesPerDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ActiveGaugesPerDenomRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ActiveGaugesPerDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACTIVE_GAUGES_PER_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.ActiveGaugesPerDenomResponse")] impl :: prost_wkt :: MessageSerde for ActiveGaugesPerDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "ActiveGaugesPerDenomResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.ActiveGaugesPerDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPCOMING_GAUGES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.UpcomingGaugesRequest")] impl :: prost_wkt :: MessageSerde for UpcomingGaugesRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "UpcomingGaugesRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.UpcomingGaugesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPCOMING_GAUGES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.UpcomingGaugesResponse")] impl :: prost_wkt :: MessageSerde for UpcomingGaugesResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "UpcomingGaugesResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.UpcomingGaugesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPCOMING_GAUGES_PER_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.UpcomingGaugesPerDenomRequest")] impl :: prost_wkt :: MessageSerde for UpcomingGaugesPerDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "UpcomingGaugesPerDenomRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.UpcomingGaugesPerDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPCOMING_GAUGES_PER_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.UpcomingGaugesPerDenomResponse")] impl :: prost_wkt :: MessageSerde for UpcomingGaugesPerDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "UpcomingGaugesPerDenomResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.UpcomingGaugesPerDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_REWARDS_EST_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.RewardsEstRequest")] impl :: prost_wkt :: MessageSerde for RewardsEstRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "RewardsEstRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.RewardsEstRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_REWARDS_EST_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.RewardsEstResponse")] impl :: prost_wkt :: MessageSerde for RewardsEstResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "RewardsEstResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.RewardsEstResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_LOCKABLE_DURATIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.QueryLockableDurationsRequest")] impl :: prost_wkt :: MessageSerde for QueryLockableDurationsRequest { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "QueryLockableDurationsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.QueryLockableDurationsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_LOCKABLE_DURATIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.QueryLockableDurationsResponse")] impl :: prost_wkt :: MessageSerde for QueryLockableDurationsResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "QueryLockableDurationsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.QueryLockableDurationsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_GAUGE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.MsgCreateGauge")] impl :: prost_wkt :: MessageSerde for MsgCreateGauge { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "MsgCreateGauge" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.MsgCreateGauge" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_GAUGE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.MsgCreateGaugeResponse")] impl :: prost_wkt :: MessageSerde for MsgCreateGaugeResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "MsgCreateGaugeResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.MsgCreateGaugeResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_ADD_TO_GAUGE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.MsgAddToGauge")] impl :: prost_wkt :: MessageSerde for MsgAddToGauge { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "MsgAddToGauge" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.MsgAddToGauge" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_ADD_TO_GAUGE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.incentives.MsgAddToGaugeResponse")] impl :: prost_wkt :: MessageSerde for MsgAddToGaugeResponse { fn package_name (& self) -> & 'static str { "osmosis.incentives" } fn message_name (& self) -> & 'static str { "MsgAddToGaugeResponse" } fn type_url (& self) -> & 'static str { "/osmosis.incentives.MsgAddToGaugeResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
