pub use eyre::{eyre as err, Context, Error, Report, Result, WrapErr};

pub mod metrics;
pub mod params;
pub mod signals;
pub mod spawn;
pub mod storage;
pub mod tracing;

pub const VERSION: &str = env!("HASH");

pub const CHAIN_ID: &str = "osmosis-1";
pub const FEE_DENOM: &str = "uosmo";
pub const LCD_URL: &str = "http://127.0.0.1:1317";
pub const MEKAPI_URL: &str = "http://127.0.0.1:4401";
pub const BUILDER_URL: &str = "http://fra-api-1:4413/"; // osmosis-1
pub const RPC_URL: &str = "ws://127.0.0.1:26657/websocket";

pub fn host() -> Option<String> {
    hostname::get()
        .ok()
        .and_then(|host| host.to_str().map(ToString::to_string))
}
