/// LegacyAminoPubKey specifies a public key type
/// which nests multiple public keys and a threshold,
/// it uses legacy amino address rules.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAminoPubKey {
    #[prost(uint32, tag="1")]
    pub threshold: u32,
    #[prost(message, repeated, tag="2")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LEGACY_AMINO_PUB_KEY : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/cosmos.crypto.multisig.LegacyAminoPubKey")] impl :: prost_wkt :: MessageSerde for LegacyAminoPubKey { fn package_name (& self) -> & 'static str { "cosmos.crypto.multisig" } fn message_name (& self) -> & 'static str { "LegacyAminoPubKey" } fn type_url (& self) -> & 'static str { "type.googleapis.com/cosmos.crypto.multisig.LegacyAminoPubKey" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
