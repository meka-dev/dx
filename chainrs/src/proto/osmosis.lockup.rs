/// PeriodLock is a single unit of lock by period. It's a record of locked coin
/// at a specific time. It stores owner, duration, unlock time and the amount of
/// coins locked.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodLock {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, repeated, tag="5")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCondition {
    /// type of lock query, ByLockDuration | ByLockTime
    #[prost(enumeration="LockQueryType", tag="1")]
    pub lock_query_type: i32,
    /// What token denomination are we looking for lockups of
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// valid when query condition is ByDuration
    #[prost(message, optional, tag="3")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
    /// valid when query condition is ByTime
    #[prost(message, optional, tag="4")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// SyntheticLock is a single unit of synthetic lockup
/// TODO: Change this to have
/// * underlying_lock_id
/// * synthetic_coin
/// * end_time
/// * duration
/// * owner
/// We then index synthetic locks by the denom, just like we do with normal
/// locks. Ideally we even get an interface, so we can re-use that same logic.
/// I currently have no idea how reward distribution is supposed to be working...
/// EVENTUALLY
/// we make a "constrained_coin" field, which is what the current "coins" field
/// is. Constrained coin field can be a #post-v7 feature, since we aren't
/// allowing partial unlocks of synthetic lockups.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLock {
    /// underlying native lockup id for this synthetic lockup
    #[prost(uint64, tag="1")]
    pub underlying_lock_id: u64,
    #[prost(string, tag="2")]
    pub synth_denom: ::prost::alloc::string::String,
    /// used for unbonding synthetic lockups, for active synthetic lockups, this
    /// value is set to uninitialized value
    #[prost(message, optional, tag="3")]
    pub end_time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockQueryType {
    /// Queries for locks that are longer than a certain duration
    ByDuration = 0,
    /// Queries for lockups that started before a specific time
    ByTime = 1,
}
/// GenesisState defines the lockup module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(uint64, tag="1")]
    pub last_lock_id: u64,
    #[prost(message, repeated, tag="2")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
    #[prost(message, repeated, tag="3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsResponse {
    #[prost(message, repeated, tag="1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomResponse {
    #[prost(string, tag="1")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedResponse {
    #[prost(message, optional, tag="1")]
    pub lock: ::core::option::Option<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdRequest {
    #[prost(uint64, tag="1")]
    pub lock_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdResponse {
    #[prost(message, repeated, tag="1")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomRequest {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomResponse {
    #[prost(message, repeated, tag="1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokens {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_wkt_types::Duration>,
    #[prost(message, repeated, tag="3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokensResponse {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAll {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAllResponse {
    #[prost(message, repeated, tag="1")]
    pub unlocks: ::prost::alloc::vec::Vec<PeriodLock>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlocking {
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    /// Amount of unlocking coins. Unlock all if not set.
    #[prost(message, repeated, tag="3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PERIOD_LOCK : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.PeriodLock")] impl :: prost_wkt :: MessageSerde for PeriodLock { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "PeriodLock" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.PeriodLock" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_CONDITION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.QueryCondition")] impl :: prost_wkt :: MessageSerde for QueryCondition { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "QueryCondition" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.QueryCondition" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SYNTHETIC_LOCK : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.SyntheticLock")] impl :: prost_wkt :: MessageSerde for SyntheticLock { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "SyntheticLock" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.SyntheticLock" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_BALANCE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.ModuleBalanceRequest")] impl :: prost_wkt :: MessageSerde for ModuleBalanceRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "ModuleBalanceRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.ModuleBalanceRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_BALANCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.ModuleBalanceResponse")] impl :: prost_wkt :: MessageSerde for ModuleBalanceResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "ModuleBalanceResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.ModuleBalanceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_LOCKED_AMOUNT_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.ModuleLockedAmountRequest")] impl :: prost_wkt :: MessageSerde for ModuleLockedAmountRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "ModuleLockedAmountRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.ModuleLockedAmountRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MODULE_LOCKED_AMOUNT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.ModuleLockedAmountResponse")] impl :: prost_wkt :: MessageSerde for ModuleLockedAmountResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "ModuleLockedAmountResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.ModuleLockedAmountResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_UNLOCKABLE_COINS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountUnlockableCoinsRequest")] impl :: prost_wkt :: MessageSerde for AccountUnlockableCoinsRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountUnlockableCoinsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountUnlockableCoinsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_UNLOCKABLE_COINS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountUnlockableCoinsResponse")] impl :: prost_wkt :: MessageSerde for AccountUnlockableCoinsResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountUnlockableCoinsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountUnlockableCoinsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_UNLOCKING_COINS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountUnlockingCoinsRequest")] impl :: prost_wkt :: MessageSerde for AccountUnlockingCoinsRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountUnlockingCoinsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountUnlockingCoinsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_UNLOCKING_COINS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountUnlockingCoinsResponse")] impl :: prost_wkt :: MessageSerde for AccountUnlockingCoinsResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountUnlockingCoinsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountUnlockingCoinsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_COINS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedCoinsRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedCoinsRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedCoinsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedCoinsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_COINS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedCoinsResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedCoinsResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedCoinsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedCoinsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_PAST_TIME_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedPastTimeRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedPastTimeRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedPastTimeRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedPastTimeRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_PAST_TIME_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedPastTimeResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedPastTimeResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedPastTimeResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedPastTimeResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_PAST_TIME_NOT_UNLOCKING_ONLY_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedPastTimeNotUnlockingOnlyRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedPastTimeNotUnlockingOnlyRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_PAST_TIME_NOT_UNLOCKING_ONLY_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedPastTimeNotUnlockingOnlyResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedPastTimeNotUnlockingOnlyResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_UNLOCKED_BEFORE_TIME_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountUnlockedBeforeTimeRequest")] impl :: prost_wkt :: MessageSerde for AccountUnlockedBeforeTimeRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountUnlockedBeforeTimeRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountUnlockedBeforeTimeRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_UNLOCKED_BEFORE_TIME_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountUnlockedBeforeTimeResponse")] impl :: prost_wkt :: MessageSerde for AccountUnlockedBeforeTimeResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountUnlockedBeforeTimeResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountUnlockedBeforeTimeResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_PAST_TIME_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedPastTimeDenomRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedPastTimeDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedPastTimeDenomRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedPastTimeDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_PAST_TIME_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedPastTimeDenomResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedPastTimeDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedPastTimeDenomResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedPastTimeDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LOCKED_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.LockedDenomRequest")] impl :: prost_wkt :: MessageSerde for LockedDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "LockedDenomRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.LockedDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LOCKED_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.LockedDenomResponse")] impl :: prost_wkt :: MessageSerde for LockedDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "LockedDenomResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.LockedDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LOCKED_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.LockedRequest")] impl :: prost_wkt :: MessageSerde for LockedRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "LockedRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.LockedRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LOCKED_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.LockedResponse")] impl :: prost_wkt :: MessageSerde for LockedResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "LockedResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.LockedResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SYNTHETIC_LOCKUPS_BY_LOCKUP_ID_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.SyntheticLockupsByLockupIDRequest")] impl :: prost_wkt :: MessageSerde for SyntheticLockupsByLockupIdRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "SyntheticLockupsByLockupIDRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.SyntheticLockupsByLockupIDRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SYNTHETIC_LOCKUPS_BY_LOCKUP_ID_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.SyntheticLockupsByLockupIDResponse")] impl :: prost_wkt :: MessageSerde for SyntheticLockupsByLockupIdResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "SyntheticLockupsByLockupIDResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.SyntheticLockupsByLockupIDResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_LONGER_DURATION_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedLongerDurationRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedLongerDurationRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedLongerDurationRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedLongerDurationRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_LONGER_DURATION_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedLongerDurationResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedLongerDurationResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedLongerDurationResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedLongerDurationResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_LONGER_DURATION_NOT_UNLOCKING_ONLY_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedLongerDurationNotUnlockingOnlyRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedLongerDurationNotUnlockingOnlyRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_LONGER_DURATION_NOT_UNLOCKING_ONLY_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedLongerDurationNotUnlockingOnlyResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedLongerDurationNotUnlockingOnlyResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_LONGER_DURATION_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedLongerDurationDenomRequest")] impl :: prost_wkt :: MessageSerde for AccountLockedLongerDurationDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedLongerDurationDenomRequest" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedLongerDurationDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ACCOUNT_LOCKED_LONGER_DURATION_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.AccountLockedLongerDurationDenomResponse")] impl :: prost_wkt :: MessageSerde for AccountLockedLongerDurationDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "AccountLockedLongerDurationDenomResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.AccountLockedLongerDurationDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_LOCK_TOKENS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.MsgLockTokens")] impl :: prost_wkt :: MessageSerde for MsgLockTokens { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "MsgLockTokens" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.MsgLockTokens" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_LOCK_TOKENS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.MsgLockTokensResponse")] impl :: prost_wkt :: MessageSerde for MsgLockTokensResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "MsgLockTokensResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.MsgLockTokensResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_BEGIN_UNLOCKING_ALL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.MsgBeginUnlockingAll")] impl :: prost_wkt :: MessageSerde for MsgBeginUnlockingAll { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "MsgBeginUnlockingAll" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.MsgBeginUnlockingAll" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_BEGIN_UNLOCKING_ALL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.MsgBeginUnlockingAllResponse")] impl :: prost_wkt :: MessageSerde for MsgBeginUnlockingAllResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "MsgBeginUnlockingAllResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.MsgBeginUnlockingAllResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_BEGIN_UNLOCKING : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.MsgBeginUnlocking")] impl :: prost_wkt :: MessageSerde for MsgBeginUnlocking { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "MsgBeginUnlocking" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.MsgBeginUnlocking" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_BEGIN_UNLOCKING_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.lockup.MsgBeginUnlockingResponse")] impl :: prost_wkt :: MessageSerde for MsgBeginUnlockingResponse { fn package_name (& self) -> & 'static str { "osmosis.lockup" } fn message_name (& self) -> & 'static str { "MsgBeginUnlockingResponse" } fn type_url (& self) -> & 'static str { "/osmosis.lockup.MsgBeginUnlockingResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
