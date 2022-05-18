/// PoolParams defined the parameters that will be managed by the pool
/// governance in the future. This params are not managed by the chain
/// governance. Instead they will be managed by the token holders of the pool.
/// The pool's token holders are specified in future_pool_governor.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolParams {
    #[prost(string, tag="1")]
    pub swap_fee: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub exit_fee: ::prost::alloc::string::String,
}
/// Pool is the stableswap Pool struct
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub id: u64,
    #[prost(message, optional, tag="3")]
    pub pool_params: ::core::option::Option<PoolParams>,
    /// This string specifies who will govern the pool in the future.
    /// Valid forms of this are:
    /// {token name},{duration}
    /// {duration}
    /// where {token name} if specified is the token which determines the
    /// governor, and if not specified is the LP token for this pool.duration is
    /// a time specified as 0w,1w,2w, etc. which specifies how long the token
    /// would need to be locked up to count in governance. 0w means no lockup.
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
    /// sum of all LP shares
    #[prost(message, optional, tag="5")]
    pub total_shares: ::core::option::Option<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// assets in the pool
    #[prost(message, repeated, tag="6")]
    pub pool_liquidity: ::prost::alloc::vec::Vec<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// for calculation amognst assets with different precisions
    #[prost(uint64, repeated, packed="false", tag="7")]
    pub scaling_factor: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStableswapPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pool_params: ::core::option::Option<PoolParams>,
    #[prost(message, repeated, tag="3")]
    pub initial_pool_liquidity: ::prost::alloc::vec::Vec<super::super::super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateStableswapPoolResponse {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_POOL_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.gamm.poolmodels.stableswap.v1beta1.PoolParams")] impl :: prost_wkt :: MessageSerde for PoolParams { fn package_name (& self) -> & 'static str { "osmosis.gamm.poolmodels.stableswap.v1beta1" } fn message_name (& self) -> & 'static str { "PoolParams" } fn type_url (& self) -> & 'static str { "/osmosis.gamm.poolmodels.stableswap.v1beta1.PoolParams" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_POOL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.gamm.poolmodels.stableswap.v1beta1.Pool")] impl :: prost_wkt :: MessageSerde for Pool { fn package_name (& self) -> & 'static str { "osmosis.gamm.poolmodels.stableswap.v1beta1" } fn message_name (& self) -> & 'static str { "Pool" } fn type_url (& self) -> & 'static str { "/osmosis.gamm.poolmodels.stableswap.v1beta1.Pool" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_STABLESWAP_POOL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.gamm.poolmodels.stableswap.v1beta1.MsgCreateStableswapPool")] impl :: prost_wkt :: MessageSerde for MsgCreateStableswapPool { fn package_name (& self) -> & 'static str { "osmosis.gamm.poolmodels.stableswap.v1beta1" } fn message_name (& self) -> & 'static str { "MsgCreateStableswapPool" } fn type_url (& self) -> & 'static str { "/osmosis.gamm.poolmodels.stableswap.v1beta1.MsgCreateStableswapPool" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_STABLESWAP_POOL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.gamm.poolmodels.stableswap.v1beta1.MsgCreateStableswapPoolResponse")] impl :: prost_wkt :: MessageSerde for MsgCreateStableswapPoolResponse { fn package_name (& self) -> & 'static str { "osmosis.gamm.poolmodels.stableswap.v1beta1" } fn message_name (& self) -> & 'static str { "MsgCreateStableswapPoolResponse" } fn type_url (& self) -> & 'static str { "/osmosis.gamm.poolmodels.stableswap.v1beta1.MsgCreateStableswapPoolResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
