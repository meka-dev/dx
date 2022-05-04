use std::net::SocketAddr;

use common::{CHAIN_ID, RPC_URL};
use params_derive::params;
use tendermint::chain;

const BUCKET_NAME: &str = "blocks";
const SERVICE_ACCOUNT_PATH: &str = "./service-account.json";

const MAX_PARALLEL_REQUESTS: u64 = 32;

pub const BINARY: &str = env!("CARGO_PKG_NAME");
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// ETL pipeline to extract block and transaction metadata.
#[params]
#[derive(argh::FromArgs, Debug)]
pub struct Args {
    /// terra chain identifier, should be static except on hard fork (Mainnet: columbus-5, Testnet:
    /// bombay-12)
    #[argh(option, default = "CHAIN_ID.parse().unwrap()")]
    pub chain_id: chain::Id,

    /// HTTP server for params reading and manipulation. (127.0.0.1:4445)
    #[argh(option, default = "([127, 0, 0, 1], 4446).into()")]
    pub params_addr: SocketAddr,

    /// websocket endpoint for Terra's tendermint RPC (ws://127.0.0.1:26657/websocket)
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
