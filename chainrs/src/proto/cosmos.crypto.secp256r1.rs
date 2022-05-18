/// PubKey defines a secp256r1 ECDSA public key.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    /// Point on secp256r1 curve in a compressed representation as specified in section
    /// 4.3.6 of ANSI X9.62: <https://webstore.ansi.org/standards/ascx9/ansix9621998>
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// PrivKey defines a secp256r1 ECDSA private key.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    /// secret number serialized using big-endian encoding
    #[prost(bytes="vec", tag="1")]
    pub secret: ::prost::alloc::vec::Vec<u8>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PUB_KEY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.crypto.secp256r1.PubKey")] impl :: prost_wkt :: MessageSerde for PubKey { fn package_name (& self) -> & 'static str { "cosmos.crypto.secp256r1" } fn message_name (& self) -> & 'static str { "PubKey" } fn type_url (& self) -> & 'static str { "/cosmos.crypto.secp256r1.PubKey" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PRIV_KEY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.crypto.secp256r1.PrivKey")] impl :: prost_wkt :: MessageSerde for PrivKey { fn package_name (& self) -> & 'static str { "cosmos.crypto.secp256r1" } fn message_name (& self) -> & 'static str { "PrivKey" } fn type_url (& self) -> & 'static str { "/cosmos.crypto.secp256r1.PrivKey" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
