/// Snapshot contains Tendermint state sync snapshot info.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(uint32, tag="2")]
    pub format: u32,
    #[prost(uint32, tag="3")]
    pub chunks: u32,
    #[prost(bytes="vec", tag="4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub metadata: ::core::option::Option<Metadata>,
}
/// Metadata contains SDK-specific snapshot metadata.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// SHA-256 chunk hashes
    #[prost(bytes="vec", repeated, tag="1")]
    pub chunk_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SNAPSHOT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.snapshots.v1beta1.Snapshot")] impl :: prost_wkt :: MessageSerde for Snapshot { fn package_name (& self) -> & 'static str { "cosmos.base.snapshots.v1beta1" } fn message_name (& self) -> & 'static str { "Snapshot" } fn type_url (& self) -> & 'static str { "/cosmos.base.snapshots.v1beta1.Snapshot" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_METADATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.snapshots.v1beta1.Metadata")] impl :: prost_wkt :: MessageSerde for Metadata { fn package_name (& self) -> & 'static str { "cosmos.base.snapshots.v1beta1" } fn message_name (& self) -> & 'static str { "Metadata" } fn type_url (& self) -> & 'static str { "/cosmos.base.snapshots.v1beta1.Metadata" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
