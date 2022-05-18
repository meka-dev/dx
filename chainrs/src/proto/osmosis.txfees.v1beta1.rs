/// FeeToken is a struct that specifies a coin denom, and pool ID pair.
/// This marks the token as eligible for use as a tx fee asset in Osmosis.
/// Its price in osmo is derived through looking at the provided pool ID.
/// The pool ID must have osmo as one of its assets.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeToken {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pool_id: u64,
}
/// GenesisState defines the txfees module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, tag="1")]
    pub basedenom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub feetokens: ::prost::alloc::vec::Vec<FeeToken>,
}
/// UpdateFeeTokenProposal is a gov Content type for adding a new whitelisted fee
/// token. It must specify a denom along with gamm pool ID to use as a spot price
/// calculator. It can be used to add a new denom to the whitelist It can also be
/// used to update the Pool to associate with the denom. If Pool ID is set to 0,
/// it will remove the denom from the whitelisted set.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeeTokenProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub feetoken: ::core::option::Option<FeeToken>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeTokensRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeTokensResponse {
    #[prost(message, repeated, tag="1")]
    pub fee_tokens: ::prost::alloc::vec::Vec<FeeToken>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomPoolIdRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomPoolIdResponse {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseDenomRequest {
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseDenomResponse {
    #[prost(string, tag="1")]
    pub base_denom: ::prost::alloc::string::String,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_FEE_TOKEN : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.FeeToken")] impl :: prost_wkt :: MessageSerde for FeeToken { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "FeeToken" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.FeeToken" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_UPDATE_FEE_TOKEN_PROPOSAL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.UpdateFeeTokenProposal")] impl :: prost_wkt :: MessageSerde for UpdateFeeTokenProposal { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "UpdateFeeTokenProposal" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.UpdateFeeTokenProposal" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_FEE_TOKENS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.QueryFeeTokensRequest")] impl :: prost_wkt :: MessageSerde for QueryFeeTokensRequest { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "QueryFeeTokensRequest" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.QueryFeeTokensRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_FEE_TOKENS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.QueryFeeTokensResponse")] impl :: prost_wkt :: MessageSerde for QueryFeeTokensResponse { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "QueryFeeTokensResponse" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.QueryFeeTokensResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_DENOM_POOL_ID_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.QueryDenomPoolIdRequest")] impl :: prost_wkt :: MessageSerde for QueryDenomPoolIdRequest { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "QueryDenomPoolIdRequest" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.QueryDenomPoolIdRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_DENOM_POOL_ID_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.QueryDenomPoolIdResponse")] impl :: prost_wkt :: MessageSerde for QueryDenomPoolIdResponse { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "QueryDenomPoolIdResponse" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.QueryDenomPoolIdResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_BASE_DENOM_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.QueryBaseDenomRequest")] impl :: prost_wkt :: MessageSerde for QueryBaseDenomRequest { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "QueryBaseDenomRequest" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.QueryBaseDenomRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_BASE_DENOM_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.txfees.v1beta1.QueryBaseDenomResponse")] impl :: prost_wkt :: MessageSerde for QueryBaseDenomResponse { fn package_name (& self) -> & 'static str { "osmosis.txfees.v1beta1" } fn message_name (& self) -> & 'static str { "QueryBaseDenomResponse" } fn type_url (& self) -> & 'static str { "/osmosis.txfees.v1beta1.QueryBaseDenomResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
