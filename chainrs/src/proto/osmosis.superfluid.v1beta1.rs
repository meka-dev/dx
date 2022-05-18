/// SetSuperfluidAssetsProposal is a gov Content type to update the superfluid
/// assets
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSuperfluidAssetsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub assets: ::prost::alloc::vec::Vec<super::SuperfluidAsset>,
}
/// RemoveSuperfluidAssetsProposal is a gov Content type to remove the superfluid
/// assets by denom
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSuperfluidAssetsProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub superfluid_asset_denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SET_SUPERFLUID_ASSETS_PROPOSAL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.superfluid.v1beta1.SetSuperfluidAssetsProposal")] impl :: prost_wkt :: MessageSerde for SetSuperfluidAssetsProposal { fn package_name (& self) -> & 'static str { "osmosis.superfluid.v1beta1" } fn message_name (& self) -> & 'static str { "SetSuperfluidAssetsProposal" } fn type_url (& self) -> & 'static str { "/osmosis.superfluid.v1beta1.SetSuperfluidAssetsProposal" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_REMOVE_SUPERFLUID_ASSETS_PROPOSAL : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/osmosis.superfluid.v1beta1.RemoveSuperfluidAssetsProposal")] impl :: prost_wkt :: MessageSerde for RemoveSuperfluidAssetsProposal { fn package_name (& self) -> & 'static str { "osmosis.superfluid.v1beta1" } fn message_name (& self) -> & 'static str { "RemoveSuperfluidAssetsProposal" } fn type_url (& self) -> & 'static str { "/osmosis.superfluid.v1beta1.RemoveSuperfluidAssetsProposal" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
