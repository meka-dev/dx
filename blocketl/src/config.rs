use std::net::SocketAddr;

use common::RPC_URL;
use params_derive::params;
use tendermint::chain;

use crate::venue::Venue;

const CHAIN_ID: &str = "osmosis-1";
const VENUE: Venue = Venue::Osmosis;

const BUCKET_NAME: &str = "blocks";
const SERVICE_ACCOUNT_PATH: &str = "./service-account.json";

const MAX_PARALLEL_REQUESTS: u64 = 32;

pub const BINARY: &str = env!("CARGO_PKG_NAME");
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// ETL pipeline to extract block and transaction metadata.
#[params]
#[derive(argh::FromArgs, Debug)]
pub struct Args {
    /// comsmos network.
    #[argh(option, default = "VENUE")]
    pub venue: Venue,

    /// chain identifier, should be static except on hard fork.
    #[argh(option, default = "CHAIN_ID.parse().unwrap()")]
    pub chain_id: chain::Id,

    /// HTTP server for operational endpoints. (127.0.0.1:4445)
    #[argh(option, default = "([127, 0, 0, 1], 4446).into()")]
    pub op_addr: SocketAddr,

    /// RPC endpoint serving Tendermint websocket (ws://127.0.0.1:26657/websocket)
    #[argh(option, default = "RPC_URL.to_string()")]
    pub rpc_url: String,

    /// GCS bucket for metadata storage
    #[argh(option, default = "BUCKET_NAME.to_string()")]
    pub bucket: String,

    /// path to file containing GCS service account information
    #[argh(option, default = "SERVICE_ACCOUNT_PATH.to_string()")]
    pub service_account: String,

    /// amount of parallel requests to other components
    #[argh(option, default = "MAX_PARALLEL_REQUESTS")]
    pub max_parallel_requests: u64,
}
