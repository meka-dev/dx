/// GenesisState defines the authz module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag="1")]
    pub authorization: ::prost::alloc::vec::Vec<GrantAuthorization>,
}
/// GrantAuthorization defines the GenesisState/GrantAuthorization type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GrantAuthorization {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub authorization: ::core::option::Option<::prost_wkt_types::Any>,
    #[prost(message, optional, tag="4")]
    pub expiration: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// GenericAuthorization gives the grantee unrestricted permissions to execute
/// the provided method on behalf of the granter's account.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericAuthorization {
    /// Msg, identified by it's type URL, to grant unrestricted permissions to execute
    #[prost(string, tag="1")]
    pub msg: ::prost::alloc::string::String,
}
/// Grant gives permissions to execute
/// the provide method with expiration time.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    #[prost(message, optional, tag="1")]
    pub authorization: ::core::option::Option<::prost_wkt_types::Any>,
    #[prost(message, optional, tag="2")]
    pub expiration: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
/// EventGrant is emitted on Msg/Grant
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventGrant {
    /// Msg type URL for which an autorization is granted
    #[prost(string, tag="2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag="3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag="4")]
    pub grantee: ::prost::alloc::string::String,
}
/// EventRevoke is emitted on Msg/Revoke
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRevoke {
    /// Msg type URL for which an autorization is revoked
    #[prost(string, tag="2")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// Granter account address
    #[prost(string, tag="3")]
    pub granter: ::prost::alloc::string::String,
    /// Grantee account address
    #[prost(string, tag="4")]
    pub grantee: ::prost::alloc::string::String,
}
/// QueryGrantsRequest is the request type for the Query/Grants RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsRequest {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    /// Optional, msg_type_url, when set, will query only grants matching given msg type.
    #[prost(string, tag="3")]
    pub msg_type_url: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag="4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGrantsResponse is the response type for the Query/Authorizations RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantsResponse {
    /// authorizations is a list of grants granted for grantee by granter.
    #[prost(message, repeated, tag="1")]
    pub grants: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgGrant is a request type for Grant method. It declares authorization to the grantee
/// on behalf of the granter with the provided expiration time.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrant {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub grant: ::core::option::Option<Grant>,
}
/// MsgExecResponse defines the Msg/MsgExecResponse response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExec {
    #[prost(string, tag="1")]
    pub grantee: ::prost::alloc::string::String,
    /// Authorization Msg requests to execute. Each msg must implement Authorization interface
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    #[prost(message, repeated, tag="2")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
}
/// MsgGrantResponse defines the Msg/MsgGrant response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantResponse {
}
/// MsgRevoke revokes any authorization with the provided sdk.Msg type on the
/// granter's account with that has been granted to the grantee.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevoke {
    #[prost(string, tag="1")]
    pub granter: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub msg_type_url: ::prost::alloc::string::String,
}
/// MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeResponse {
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GRANT_AUTHORIZATION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.GrantAuthorization")] impl :: prost_wkt :: MessageSerde for GrantAuthorization { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "GrantAuthorization" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.GrantAuthorization" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENERIC_AUTHORIZATION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.GenericAuthorization")] impl :: prost_wkt :: MessageSerde for GenericAuthorization { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "GenericAuthorization" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.GenericAuthorization" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GRANT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.Grant")] impl :: prost_wkt :: MessageSerde for Grant { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "Grant" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.Grant" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EVENT_GRANT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.EventGrant")] impl :: prost_wkt :: MessageSerde for EventGrant { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "EventGrant" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.EventGrant" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EVENT_REVOKE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.EventRevoke")] impl :: prost_wkt :: MessageSerde for EventRevoke { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "EventRevoke" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.EventRevoke" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.QueryGrantsRequest")] impl :: prost_wkt :: MessageSerde for QueryGrantsRequest { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "QueryGrantsRequest" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.QueryGrantsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_GRANTS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.QueryGrantsResponse")] impl :: prost_wkt :: MessageSerde for QueryGrantsResponse { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "QueryGrantsResponse" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.QueryGrantsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_GRANT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.MsgGrant")] impl :: prost_wkt :: MessageSerde for MsgGrant { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "MsgGrant" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.MsgGrant" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_EXEC_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.MsgExecResponse")] impl :: prost_wkt :: MessageSerde for MsgExecResponse { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "MsgExecResponse" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.MsgExecResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_EXEC : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.MsgExec")] impl :: prost_wkt :: MessageSerde for MsgExec { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "MsgExec" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.MsgExec" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_GRANT_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.MsgGrantResponse")] impl :: prost_wkt :: MessageSerde for MsgGrantResponse { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "MsgGrantResponse" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.MsgGrantResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_REVOKE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.MsgRevoke")] impl :: prost_wkt :: MessageSerde for MsgRevoke { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "MsgRevoke" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.MsgRevoke" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_REVOKE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.authz.v1beta1.MsgRevokeResponse")] impl :: prost_wkt :: MessageSerde for MsgRevokeResponse { fn package_name (& self) -> & 'static str { "cosmos.authz.v1beta1" } fn message_name (& self) -> & 'static str { "MsgRevokeResponse" } fn type_url (& self) -> & 'static str { "/cosmos.authz.v1beta1.MsgRevokeResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
