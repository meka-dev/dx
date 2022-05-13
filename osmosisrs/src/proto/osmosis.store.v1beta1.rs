#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(message, repeated, tag="1")]
    pub children: ::prost::alloc::vec::Vec<Child>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Child {
    #[prost(bytes="vec", tag="1")]
    pub index: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub accumulation: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Leaf {
    #[prost(message, optional, tag="1")]
    pub leaf: ::core::option::Option<Child>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_NODE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.store.v1beta1.Node")] impl :: prost_wkt :: MessageSerde for Node { fn package_name (& self) -> & 'static str { "osmosis.store.v1beta1" } fn message_name (& self) -> & 'static str { "Node" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.store.v1beta1.Node" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_CHILD : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.store.v1beta1.Child")] impl :: prost_wkt :: MessageSerde for Child { fn package_name (& self) -> & 'static str { "osmosis.store.v1beta1" } fn message_name (& self) -> & 'static str { "Child" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.store.v1beta1.Child" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_LEAF : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "type.googleapis.com/osmosis.store.v1beta1.Leaf")] impl :: prost_wkt :: MessageSerde for Leaf { fn package_name (& self) -> & 'static str { "osmosis.store.v1beta1" } fn message_name (& self) -> & 'static str { "Leaf" } fn type_url (& self) -> & 'static str { "type.googleapis.com/osmosis.store.v1beta1.Leaf" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
