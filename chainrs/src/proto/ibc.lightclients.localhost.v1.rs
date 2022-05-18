/// ClientState defines a loopback (localhost) client. It requires (read-only)
/// access to keys outside the client prefix.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// self chain ID
    #[prost(string, tag="1")]
    pub chain_id: ::prost::alloc::string::String,
    /// self latest block height
    #[prost(message, optional, tag="2")]
    pub height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CLIENT_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.lightclients.localhost.v1.ClientState")] impl :: prost_wkt :: MessageSerde for ClientState { fn package_name (& self) -> & 'static str { "ibc.lightclients.localhost.v1" } fn message_name (& self) -> & 'static str { "ClientState" } fn type_url (& self) -> & 'static str { "/ibc.lightclients.localhost.v1.ClientState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
