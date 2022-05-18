/// Minter represents the minting state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Minter {
    /// current annual inflation rate
    #[prost(string, tag="1")]
    pub inflation: ::prost::alloc::string::String,
    /// current annual expected provisions
    #[prost(string, tag="2")]
    pub annual_provisions: ::prost::alloc::string::String,
}
/// Params holds parameters for the mint module.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// type of coin to mint
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    /// maximum annual change in inflation rate
    #[prost(string, tag="2")]
    pub inflation_rate_change: ::prost::alloc::string::String,
    /// maximum inflation rate
    #[prost(string, tag="3")]
    pub inflation_max: ::prost::alloc::string::String,
    /// minimum inflation rate
    #[prost(string, tag="4")]
    pub inflation_min: ::prost::alloc::string::String,
    /// goal of percent bonded atoms
    #[prost(string, tag="5")]
    pub goal_bonded: ::prost::alloc::string::String,
    /// expected blocks per year
    #[prost(uint64, tag="6")]
    pub blocks_per_year: u64,
}
/// GenesisState defines the mint module's genesis state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// minter is a space for holding current inflation information.
    #[prost(message, optional, tag="1")]
    pub minter: ::core::option::Option<Minter>,
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryInflationRequest is the request type for the Query/Inflation RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationRequest {
}
/// QueryInflationResponse is the response type for the Query/Inflation RPC
/// method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationResponse {
    /// inflation is the current minting inflation value.
    #[prost(bytes="vec", tag="1")]
    pub inflation: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAnnualProvisionsRequest is the request type for the
/// Query/AnnualProvisions RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnualProvisionsRequest {
}
/// QueryAnnualProvisionsResponse is the response type for the
/// Query/AnnualProvisions RPC method.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnualProvisionsResponse {
    /// annual_provisions is the current minting annual provisions value.
    #[prost(bytes="vec", tag="1")]
    pub annual_provisions: ::prost::alloc::vec::Vec<u8>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MINTER : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.Minter")] impl :: prost_wkt :: MessageSerde for Minter { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "Minter" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.Minter" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PARAMS : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.Params")] impl :: prost_wkt :: MessageSerde for Params { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "Params" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.Params" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GENESIS_STATE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.GenesisState")] impl :: prost_wkt :: MessageSerde for GenesisState { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "GenesisState" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.GenesisState" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.QueryParamsRequest")] impl :: prost_wkt :: MessageSerde for QueryParamsRequest { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsRequest" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.QueryParamsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_PARAMS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.QueryParamsResponse")] impl :: prost_wkt :: MessageSerde for QueryParamsResponse { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryParamsResponse" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.QueryParamsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_INFLATION_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.QueryInflationRequest")] impl :: prost_wkt :: MessageSerde for QueryInflationRequest { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryInflationRequest" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.QueryInflationRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_INFLATION_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.QueryInflationResponse")] impl :: prost_wkt :: MessageSerde for QueryInflationResponse { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryInflationResponse" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.QueryInflationResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ANNUAL_PROVISIONS_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.QueryAnnualProvisionsRequest")] impl :: prost_wkt :: MessageSerde for QueryAnnualProvisionsRequest { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAnnualProvisionsRequest" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.QueryAnnualProvisionsRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_QUERY_ANNUAL_PROVISIONS_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.mint.v1beta1.QueryAnnualProvisionsResponse")] impl :: prost_wkt :: MessageSerde for QueryAnnualProvisionsResponse { fn package_name (& self) -> & 'static str { "cosmos.mint.v1beta1" } fn message_name (& self) -> & 'static str { "QueryAnnualProvisionsResponse" } fn type_url (& self) -> & 'static str { "/cosmos.mint.v1beta1.QueryAnnualProvisionsResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
