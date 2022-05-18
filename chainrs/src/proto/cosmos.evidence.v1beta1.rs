/// GenesisState defines the evidence module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// evidence defines all the evidence at genesis.
    #[prost(message, repeated, tag="1")]
    pub evidence: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
}
/// Equivocation implements the Evidence interface and defines evidence of double
/// signing misbehavior.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Equivocation {
    #[prost(int64, tag="1")]
    pub height: i64,
    #[prost(message, optional, tag="2")]
    pub time: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(int64, tag="3")]
    pub power: i64,
    #[prost(string, tag="4")]
    pub consensus_address: ::prost::alloc::string::String,
}
/// QueryEvidenceRequest is the request type for the Query/Evidence RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEvidenceRequest {
    /// evidence_hash defines the hash of the requested evidence.
    #[prost(bytes="vec", tag="1")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
}
/// QueryEvidenceResponse is the response type for the Query/Evidence RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEvidenceResponse {
    /// evidence returns the requested evidence.
    #[prost(message, optional, tag="1")]
    pub evidence: ::core::option::Option<::prost_wkt_types::Any>,
}
/// QueryEvidenceRequest is the request type for the Query/AllEvidence RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEvidenceRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllEvidenceResponse is the response type for the Query/AllEvidence RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllEvidenceResponse {
    /// evidence returns all evidences.
    #[prost(message, repeated, tag="1")]
    pub evidence: ::prost::alloc::vec::Vec<::prost_wkt_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgSubmitEvidence represents a message that supports submitting arbitrary
/// Evidence of misbehavior such as equivocation or counterfactual signing.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidence {
    #[prost(string, tag="1")]
    pub submitter: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub evidence: ::core::option::Option<::prost_wkt_types::Any>,
}
/// MsgSubmitEvidenceResponse defines the Msg/SubmitEvidence response type.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitEvidenceResponse {
    /// hash defines the hash of the evidence.
    #[prost(bytes="vec", tag="4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_EQUIVOCATION : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.Equivocation")] impl :: prost_wkt :: MessageSerde for Equivocation { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "Equivocation" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.Equivocation" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_EVIDENCE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.QueryEvidenceRequest")] impl :: prost_wkt :: MessageSerde for QueryEvidenceRequest { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "QueryEvidenceRequest" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.QueryEvidenceRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_EVIDENCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.QueryEvidenceResponse")] impl :: prost_wkt :: MessageSerde for QueryEvidenceResponse { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "QueryEvidenceResponse" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.QueryEvidenceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ALL_EVIDENCE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.QueryAllEvidenceRequest")] impl :: prost_wkt :: MessageSerde for QueryAllEvidenceRequest { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAllEvidenceRequest" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.QueryAllEvidenceRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ALL_EVIDENCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.QueryAllEvidenceResponse")] impl :: prost_wkt :: MessageSerde for QueryAllEvidenceResponse { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAllEvidenceResponse" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.QueryAllEvidenceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUBMIT_EVIDENCE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.MsgSubmitEvidence")] impl :: prost_wkt :: MessageSerde for MsgSubmitEvidence { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "MsgSubmitEvidence" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.MsgSubmitEvidence" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_SUBMIT_EVIDENCE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.evidence.v1beta1.MsgSubmitEvidenceResponse")] impl :: prost_wkt :: MessageSerde for MsgSubmitEvidenceResponse { fn package_name (& self) -> & 'static str { "cosmos.evidence.v1beta1" } fn message_name (& self) -> & 'static str { "MsgSubmitEvidenceResponse" } fn type_url (& self) -> & 'static str { "/cosmos.evidence.v1beta1.MsgSubmitEvidenceResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
