use std::{net::SocketAddr, time::Duration};

use common::Result;
use params_derive::params;

const API_SOCKET_PATH: &str = "/tmp/mtek-collector.sock";

const FLUSH_QUEUE_DEPTH: usize = 1024;
const INGEST_QUEUE_DEPTH: usize = 1024;
const SEGMENT_BYTE_LIMIT: usize = 16 * 1024 * 1024;
const SEGMENT_COUNT_LIMIT: usize = 10_000;

const FLUSH_INTERVAL: Duration = Duration::from_millis(500);

const BUCKET_NAME: &str = "dx_accounting";
const SERVICE_ACCOUNT_PATH: &str = "./service-account.json";

pub const BINARY: &str = env!("CARGO_PKG_NAME");
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// General purpose event sink which shuttles of to a storage backend.
#[params]
#[derive(argh::FromArgs, Debug)]
pub struct Args {
    /// HTTP api server.  (127.0.0.1:4430)
    #[argh(option, default = "API_SOCKET_PATH.to_string()")]
    pub api_path: String,

    /// depth of the queue for event ingestion
    #[argh(option, default = "INGEST_QUEUE_DEPTH")]
    pub ingest_queue_depth: usize,

    /// depth of the queue for flushed segments ready to be stored
    #[argh(option, default = "FLUSH_QUEUE_DEPTH")]
    pub flush_queue_depth: usize,

    /// depth of the queue for schedule log store operations
    #[argh(option, default = "FLUSH_INTERVAL", from_str_fn(parse_duration))]
    pub flush_interval: Duration,

    /// maximum size in bytes for an open segment before it will be shuttled of
    #[argh(option, default = "SEGMENT_BYTE_LIMIT")]
    pub segment_byte_limit: usize,

    /// maximum size in number of events for an open segment before it will be shuttled of
    #[argh(option, default = "SEGMENT_COUNT_LIMIT")]
    pub segment_count_limit: usize,

    /// HTTP server for operational endpoints. (127.0.0.1:4431)
    #[argh(option, default = "([127, 0, 0, 1], 4445).into()")]
    pub op_addr: SocketAddr,

    /// GCS bucket for event storage
    #[argh(option, default = "BUCKET_NAME.to_string()")]
    pub bucket: String,

    /// path to file containing GCS service account information
    #[argh(option, default = "SERVICE_ACCOUNT_PATH.to_string()")]
    pub service_account: String,
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    value
        .parse::<humantime::Duration>()
        .map(Into::into)
        .map_err(|err| format!("failed to parse '{}' as duration: {}", value, err))
}
