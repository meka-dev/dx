/// BasicAllowance implements Allowance with a one-time grant of tokens
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicAllowance {
    /// spend_limit specifies the maximum amount of tokens that can be spent
    /// by this allowance and will be updated as tokens are spent. If it is
    /// empty, there is no spend limit and any amount of coins can be spent.
    #[prost(message, repeated, tag="1")]
    pub spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// expiration specifies an optional time when this allowance expires
    #[prost(message, optional, tag="2")]
    pub expiration: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// PeriodicAllowance extends Allowance to allow for both a maximum cap,
/// as well as a limit per time period.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodicAllowance {
    /// basic specifies a struct of `BasicAllowance`
    #[prost(message, optional, tag="1")]
    pub basic: ::core::option::Option<BasicAllowance>,
    /// period specifies the time duration in which period_spend_limit coins can
    /// be spent before that allowance is reset
    #[prost(message, optional, tag="2")]
    pub period: ::core::option::Option<::prost_wkt_types::Duration>,
    /// period_spend_limit specifies the maximum number of coins that can be spent
    /// in the period
    #[prost(message, repeated, tag="3")]
    pub period_spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// period_can_spend is the number of coins left to be spent before the period_reset time
    #[prost(message, repeated, tag="4")]
    pub period_can_spend: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// period_reset is the time at which this period resets and a new one begins,
    /// it is calculated from the start time of the first transaction after the
    /// last period ended
    #[prost(message, optional, tag="5")]
    pub period_reset: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// AllowedMsgAllowance creates allowance only for specified message types.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedMsgAllowance {
    /// allowance can be any of basic and filtered fee allowance.
    #[prost(message, optional, tag="1")]
    pub allowance: ::core::option::Option<::prost_wkt_types::Any>,
    /// allowed_messages are the messages for which the grantee has the access.
    #[prost(string, repeated, tag="2")]
    pub allowed_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Grant is stored in the KVStore to record a grant with full context
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic and filtered fee allowance.
    #[prost(message, optional, tag="3")]
    pub allowance: ::core::option::Option<::prost_wkt_types::Any>,
}
/// GenesisState contains a set of fee allowances, persisted from the store
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub allowances: ::prost::alloc::vec::Vec<Grant>,
}
/// QueryAllowanceRequest is the request type for the Query/Allowance RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowanceRequest {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
}
/// QueryAllowanceResponse is the response type for the Query/Allowance RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowanceResponse {
    /// allowance is a allowance granted for grantee by granter.
    #[prost(message, optional, tag="1")]
    pub allowance: ::core::option::Option<Grant>,
}
/// QueryAllowancesRequest is the request type for the Query/Allowances RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowancesRequest {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllowancesResponse is the response type for the Query/Allowances RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowancesResponse {
    /// allowances are allowance's granted for grantee by granter.
    #[prost(message, repeated, tag="1")]
    pub allowances: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgGrantAllowance adds permission for Grantee to spend up to Allowance
/// of fees from the account of Granter.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic and filtered fee allowance.
    #[prost(message, optional, tag="3")]
    pub allowance: ::core::option::Option<::prost_wkt_types::Any>,
}
/// MsgGrantAllowanceResponse defines the Msg/GrantAllowanceResponse response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantAllowanceResponse {
}
/// MsgRevokeAllowance removes any existing Allowance from Granter to Grantee.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
}
/// MsgRevokeAllowanceResponse defines the Msg/RevokeAllowanceResponse response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeAllowanceResponse {
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_BASIC_ALLOWANCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.BasicAllowance")] impl :: prost_wkt :: MessageSerde for BasicAllowance { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "BasicAllowance" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.BasicAllowance" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PERIODIC_ALLOWANCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.PeriodicAllowance")] impl :: prost_wkt :: MessageSerde for PeriodicAllowance { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "PeriodicAllowance" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.PeriodicAllowance" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ALLOWED_MSG_ALLOWANCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.AllowedMsgAllowance")] impl :: prost_wkt :: MessageSerde for AllowedMsgAllowance { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "AllowedMsgAllowance" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.AllowedMsgAllowance" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GRANT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.Grant")] impl :: prost_wkt :: MessageSerde for Grant { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "Grant" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.Grant" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ALLOWANCE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.QueryAllowanceRequest")] impl :: prost_wkt :: MessageSerde for QueryAllowanceRequest { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAllowanceRequest" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.QueryAllowanceRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ALLOWANCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.QueryAllowanceResponse")] impl :: prost_wkt :: MessageSerde for QueryAllowanceResponse { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAllowanceResponse" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.QueryAllowanceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ALLOWANCES_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.QueryAllowancesRequest")] impl :: prost_wkt :: MessageSerde for QueryAllowancesRequest { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAllowancesRequest" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.QueryAllowancesRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ALLOWANCES_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.QueryAllowancesResponse")] impl :: prost_wkt :: MessageSerde for QueryAllowancesResponse { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAllowancesResponse" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.QueryAllowancesResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_GRANT_ALLOWANCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.MsgGrantAllowance")] impl :: prost_wkt :: MessageSerde for MsgGrantAllowance { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "MsgGrantAllowance" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.MsgGrantAllowance" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_GRANT_ALLOWANCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.MsgGrantAllowanceResponse")] impl :: prost_wkt :: MessageSerde for MsgGrantAllowanceResponse { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "MsgGrantAllowanceResponse" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.MsgGrantAllowanceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_REVOKE_ALLOWANCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.MsgRevokeAllowance")] impl :: prost_wkt :: MessageSerde for MsgRevokeAllowance { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "MsgRevokeAllowance" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.MsgRevokeAllowance" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_REVOKE_ALLOWANCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.feegrant.v1beta1.MsgRevokeAllowanceResponse")] impl :: prost_wkt :: MessageSerde for MsgRevokeAllowanceResponse { fn package_name (& self) -> & 'static str { "cosmos.feegrant.v1beta1" } fn message_name (& self) -> & 'static str { "MsgRevokeAllowanceResponse" } fn type_url (& self) -> & 'static str { "/cosmos.feegrant.v1beta1.MsgRevokeAllowanceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
