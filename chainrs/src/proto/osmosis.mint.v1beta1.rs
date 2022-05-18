/// Minter represents the minting state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minter {
    /// current epoch provisions
    #[prost(string, tag="1")]
    pub epoch_provisions: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeightedAddress {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub weight: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionProportions {
    /// staking defines the proportion of the minted minted_denom that is to be
    /// allocated as staking rewards.
    #[prost(string, tag="1")]
    pub staking: ::prost::alloc::string::String,
    /// pool_incentives defines the proportion of the minted minted_denom that is
    /// to be allocated as pool incentives.
    #[prost(string, tag="2")]
    pub pool_incentives: ::prost::alloc::string::String,
    /// developer_rewards defines the proportion of the minted minted_denom that is
    /// to be allocated to developer rewards address.
    #[prost(string, tag="3")]
    pub developer_rewards: ::prost::alloc::string::String,
    /// community_pool defines the proportion of the minted minted_denom that is
    /// to be allocated to the community pool.
    #[prost(string, tag="4")]
    pub community_pool: ::prost::alloc::string::String,
}
/// Params holds parameters for the mint module.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// epoch provisions from the first epoch
    #[prost(string, tag="2")]
    pub genesis_epoch_provisions: ::prost::alloc::string::String,
    /// mint epoch identifier
    #[prost(string, tag="3")]
    pub epoch_identifier: ::prost::alloc::string::String,
    /// number of epochs take to reduce rewards
    #[prost(int64, tag="4")]
    pub reduction_period_in_epochs: i64,
    /// reduction multiplier to execute on each period
    #[prost(string, tag="5")]
    pub reduction_factor: ::prost::alloc::string::String,
    /// distribution_proportions defines the proportion of the minted denom
    #[prost(message, optional, tag="6")]
    pub distribution_proportions: ::core::option::Option<DistributionProportions>,
    /// address to receive developer rewards
    #[prost(message, repeated, tag="7")]
    pub weighted_developer_rewards_receivers: ::prost::alloc::vec::Vec<WeightedAddress>,
    /// start epoch to distribute minting rewards
    #[prost(int64, tag="8")]
    pub minting_rewards_distribution_start_epoch: i64,
}
/// GenesisState defines the mint module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// minter is a space for holding current rewards information.
    #[prost(message, optional, tag="1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
    /// current halven period start epoch
    #[prost(int64, tag="3")]
    pub halven_started_epoch: i64,
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
/// QueryEpochProvisionsRequest is the request type for the
/// Query/EpochProvisions RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochProvisionsRequest {
}
/// QueryEpochProvisionsResponse is the response type for the
/// Query/EpochProvisions RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEpochProvisionsResponse {
    /// epoch_provisions is the current minting per epoch provisions value.
    #[prost(bytes="vec", tag="1")]
    pub epoch_provisions: ::prost::alloc::vec::Vec<u8>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MINTER : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.Minter")] impl :: prost_wkt :: MessageSerde for Minter { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "Minter" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.Minter" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_WEIGHTED_ADDRESS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.WeightedAddress")] impl :: prost_wkt :: MessageSerde for WeightedAddress { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "WeightedAddress" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.WeightedAddress" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_DISTRIBUTION_PROPORTIONS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.DistributionProportions")] impl :: prost_wkt :: MessageSerde for DistributionProportions { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "DistributionProportions" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.DistributionProportions" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.QueryParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryParamsRequest { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.QueryParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.QueryParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryParamsResponse { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.QueryParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_EPOCH_PROVISIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.QueryEpochProvisionsRequest")] impl :: prost_wkt :: MessageSerde for QueryEpochProvisionsRequest { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryEpochProvisionsRequest" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.QueryEpochProvisionsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_EPOCH_PROVISIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.mint.v1beta1.QueryEpochProvisionsResponse")] impl :: prost_wkt :: MessageSerde for QueryEpochProvisionsResponse { fn package_name (& self) -> & 'static str { "osmosis.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryEpochProvisionsResponse" } fn type_url (& self) -> & 'static str { "/osmosis.mint.v1beta1.QueryEpochProvisionsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
