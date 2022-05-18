/// PageRequest is to be embedded in gRPC request messages for efficient
/// pagination. Ex:
///
///  message SomeRequest {
///          Foo some_parameter = 1;
///          PageRequest pagination = 2;
///  }
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRequest {
    /// key is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// offset is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    #[prost(uint64, tag="2")]
    pub offset: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    #[prost(uint64, tag="3")]
    pub limit: u64,
    /// count_total is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    #[prost(bool, tag="4")]
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(bool, tag="5")]
    pub reverse: bool,
}
/// PageResponse is to be embedded in gRPC response messages where the
/// corresponding request message has used PageRequest.
///
///  message SomeResponse {
///          repeated Bar results = 1;
///          PageResponse page = 2;
///  }
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageResponse {
    /// next_key is the key to be passed to PageRequest.key to
    /// query the next page most efficiently
    #[prost(bytes="vec", tag="1")]
    pub next_key: ::prost::alloc::vec::Vec<u8>,
    /// total is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    #[prost(uint64, tag="2")]
    pub total: u64,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PAGE_REQUEST : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.query.v1beta1.PageRequest")] impl :: prost_wkt :: MessageSerde for PageRequest { fn package_name (& self) -> & 'static str { "cosmos.base.query.v1beta1" } fn message_name (& self) -> & 'static str { "PageRequest" } fn type_url (& self) -> & 'static str { "/cosmos.base.query.v1beta1.PageRequest" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_PAGE_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.query.v1beta1.PageResponse")] impl :: prost_wkt :: MessageSerde for PageResponse { fn package_name (& self) -> & 'static str { "cosmos.base.query.v1beta1" } fn message_name (& self) -> & 'static str { "PageResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.query.v1beta1.PageResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
