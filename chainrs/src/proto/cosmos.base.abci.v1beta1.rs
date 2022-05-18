/// TxResponse defines a structure containing relevant tx data and metadata. The
/// tags are stringified and the log is JSON decoded.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResponse {
    /// The block height
    #[prost(int64, tag="1")]
    pub height: i64,
    /// The transaction hash.
    #[prost(string, tag="2")]
    pub txhash: ::prost::alloc::string::String,
    /// Namespace for the Code
    #[prost(string, tag="3")]
    pub codespace: ::prost::alloc::string::String,
    /// Response code.
    #[prost(uint32, tag="4")]
    pub code: u32,
    /// Result bytes, if any.
    #[prost(string, tag="5")]
    pub data: ::prost::alloc::string::String,
    /// The output of the application's logger (raw string). May be
    /// non-deterministic.
    #[prost(string, tag="6")]
    pub raw_log: ::prost::alloc::string::String,
    /// The output of the application's logger (typed). May be non-deterministic.
    #[prost(message, repeated, tag="7")]
    pub logs: ::prost::alloc::vec::Vec<AbciMessageLog>,
    /// Additional information. May be non-deterministic.
    #[prost(string, tag="8")]
    pub info: ::prost::alloc::string::String,
    /// Amount of gas requested for transaction.
    #[prost(int64, tag="9")]
    pub gas_wanted: i64,
    /// Amount of gas consumed by transaction.
    #[prost(int64, tag="10")]
    pub gas_used: i64,
    /// The request transaction bytes.
    #[prost(message, optional, tag="11")]
    pub tx: ::core::option::Option<::prost_wkt_types::Any>,
    /// Time of the previous block. For heights > 1, it's the weighted median of
    /// the timestamps of the valid votes in the block.LastCommit. For height == 1,
    /// it's genesis time.
    #[prost(string, tag="12")]
    pub timestamp: ::prost::alloc::string::String,
    /// Events defines all the events emitted by processing a transaction. Note,
    /// these events include those emitted by processing all the messages and those
    /// emitted from the ante handler. Whereas Logs contains the events, with
    /// additional metadata, emitted only by processing the messages.
    ///
    /// Since: cosmos-sdk 0.42.11, 0.44.5, 0.45
    #[prost(message, repeated, tag="13")]
    #[serde(skip)]
    pub events: ::prost::alloc::vec::Vec<::tendermint_proto::abci::Event>,
}
/// ABCIMessageLog defines a structure containing an indexed tx ABCI message log.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciMessageLog {
    #[prost(uint32, tag="1")]
    pub msg_index: u32,
    #[prost(string, tag="2")]
    pub log: ::prost::alloc::string::String,
    /// Events contains a slice of Event objects that were emitted during some
    /// execution.
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<StringEvent>,
}
/// StringEvent defines en Event object wrapper where all the attributes
/// contain key/value pairs that are strings instead of raw bytes.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringEvent {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
}
/// Attribute defines an attribute wrapper where the key and value are
/// strings instead of raw bytes.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
/// GasInfo defines tx execution gas context.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    #[prost(uint64, tag="1")]
    pub gas_wanted: u64,
    /// GasUsed is the amount of gas actually consumed.
    #[prost(uint64, tag="2")]
    pub gas_used: u64,
}
/// Result is the union of ResponseFormat and ResponseCheckTx.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseResult {
    /// Data is any data returned from message or handler execution. It MUST be
    /// length prefixed in order to separate data from multiple message executions.
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Log contains the log information from message or handler execution.
    #[prost(string, tag="2")]
    pub log: ::prost::alloc::string::String,
    /// Events contains a slice of Event objects that were emitted during message
    /// or handler execution.
    #[prost(message, repeated, tag="3")]
    #[serde(skip)]
    pub events: ::prost::alloc::vec::Vec<::tendermint_proto::abci::Event>,
}
/// SimulationResponse defines the response generated when a transaction is
/// successfully simulated.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulationResponse {
    #[prost(message, optional, tag="1")]
    pub gas_info: ::core::option::Option<GasInfo>,
    #[prost(message, optional, tag="2")]
    pub result: ::core::option::Option<ResponseResult>,
}
/// MsgData defines the data returned in a Result object during message
/// execution.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgData {
    #[prost(string, tag="1")]
    pub msg_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// TxMsgData defines a list of MsgData. A transaction will have a MsgData object
/// for each message.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxMsgData {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<MsgData>,
}
/// SearchTxsResult defines a structure for querying txs pageable
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTxsResult {
    /// Count of all txs
    #[prost(uint64, tag="1")]
    pub total_count: u64,
    /// Count of txs in current page
    #[prost(uint64, tag="2")]
    pub count: u64,
    /// Index of current page, start from 1
    #[prost(uint64, tag="3")]
    pub page_number: u64,
    /// Count of total pages
    #[prost(uint64, tag="4")]
    pub page_total: u64,
    /// Max count txs per page
    #[prost(uint64, tag="5")]
    pub limit: u64,
    /// List of txs in current page
    #[prost(message, repeated, tag="6")]
    pub txs: ::prost::alloc::vec::Vec<TxResponse>,
}

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TX_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.TxResponse")] impl :: prost_wkt :: MessageSerde for TxResponse { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "TxResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.TxResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ABCI_MESSAGE_LOG : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.ABCIMessageLog")] impl :: prost_wkt :: MessageSerde for AbciMessageLog { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "ABCIMessageLog" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.ABCIMessageLog" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_STRING_EVENT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.StringEvent")] impl :: prost_wkt :: MessageSerde for StringEvent { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "StringEvent" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.StringEvent" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_ATTRIBUTE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.Attribute")] impl :: prost_wkt :: MessageSerde for Attribute { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "Attribute" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.Attribute" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_GAS_INFO : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.GasInfo")] impl :: prost_wkt :: MessageSerde for GasInfo { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "GasInfo" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.GasInfo" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_RESULT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.Result")] impl :: prost_wkt :: MessageSerde for ResponseResult { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "Result" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.Result" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SIMULATION_RESPONSE : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.SimulationResponse")] impl :: prost_wkt :: MessageSerde for SimulationResponse { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "SimulationResponse" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.SimulationResponse" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_MSG_DATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.MsgData")] impl :: prost_wkt :: MessageSerde for MsgData { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "MsgData" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.MsgData" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_TX_MSG_DATA : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.TxMsgData")] impl :: prost_wkt :: MessageSerde for TxMsgData { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "TxMsgData" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.TxMsgData" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;

# [allow (dead_code)] const IMPL_MESSAGE_SERDE_FOR_SEARCH_TXS_RESULT : () = { use :: prost_wkt :: typetag ; # [typetag :: serde (name = "/cosmos.base.abci.v1beta1.SearchTxsResult")] impl :: prost_wkt :: MessageSerde for SearchTxsResult { fn package_name (& self) -> & 'static str { "cosmos.base.abci.v1beta1" } fn message_name (& self) -> & 'static str { "SearchTxsResult" } fn type_url (& self) -> & 'static str { "/cosmos.base.abci.v1beta1.SearchTxsResult" } fn new_instance (& self , data : Vec < u8 >) -> Result < Box < dyn :: prost_wkt :: MessageSerde > , :: prost :: DecodeError > { let mut target = Self :: default () ; :: prost :: Message :: merge (& mut target , data . as_slice ()) ? ; let erased : Box < dyn :: prost_wkt :: MessageSerde > = Box :: new (target) ; Ok (erased) } fn encoded (& self) -> Vec < u8 > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) . expect ("Failed to encode message") ; buf } fn try_encoded (& self) -> Result < Vec < u8 > , :: prost :: EncodeError > { let mut buf = Vec :: new () ; buf . reserve (:: prost :: Message :: encoded_len (self)) ; :: prost :: Message :: encode (self , & mut buf) ? ; Ok (buf) } } } ;
