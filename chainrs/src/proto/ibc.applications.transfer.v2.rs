/// FungibleTokenPacketData defines a struct for the packet payload
/// See FungibleTokenPacketData spec:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FungibleTokenPacketData {
    /// the token denomination to be transferred
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// the token amount to be transferred
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    /// the sender address
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag="4")]
    pub receiver: ::prost::alloc::string::String,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_FUNGIBLE_TOKEN_PACKET_DATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/ibc.applications.transfer.v2.FungibleTokenPacketData")] impl :: prost_wkt :: MessageSerde for FungibleTokenPacketData { fn package_name (& self) -> & 'static str { "ibc.applications.transfer.v2" } fn message_name (& self) -> & 'static str { "FungibleTokenPacketData" } fn type_url (& self) -> & 'static str { "/ibc.applications.transfer.v2.FungibleTokenPacketData" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
