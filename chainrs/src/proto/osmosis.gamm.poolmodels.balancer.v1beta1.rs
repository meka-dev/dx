/// ===================== MsgCreatePool
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBalancerPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pool_params: ::core::option::Option<super::super::super::v1beta1::PoolParams>,
    #[prost(message, repeated, tag="3")]
    pub pool_assets: ::prost::alloc::vec::Vec<super::super::super::v1beta1::PoolAsset>,
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBalancerPoolResponse {
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_BALANCER_POOL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.gamm.poolmodels.balancer.v1beta1.MsgCreateBalancerPool")] impl :: prost_wkt :: MessageSerde for MsgCreateBalancerPool { fn package_name (& self) -> & 'static str { "osmosis.gamm.poolmodels.balancer.v1beta1" } fn message_name (& self) -> & 'static str { "MsgCreateBalancerPool" } fn type_url (& self) -> & 'static str { "/osmosis.gamm.poolmodels.balancer.v1beta1.MsgCreateBalancerPool" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_CREATE_BALANCER_POOL_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.gamm.poolmodels.balancer.v1beta1.MsgCreateBalancerPoolResponse")] impl :: prost_wkt :: MessageSerde for MsgCreateBalancerPoolResponse { fn package_name (& self) -> & 'static str { "osmosis.gamm.poolmodels.balancer.v1beta1" } fn message_name (& self) -> & 'static str { "MsgCreateBalancerPoolResponse" } fn type_url (& self) -> & 'static str { "/osmosis.gamm.poolmodels.balancer.v1beta1.MsgCreateBalancerPoolResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
