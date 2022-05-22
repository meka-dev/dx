pub use eyre::{eyre as err, Context, Error, Report, Result, WrapErr};

pub mod metrics;
pub mod params;
pub mod signals;
pub mod spawn;
pub mod storage;
pub mod tracing;

pub const VERSION: &str = env!("HASH");

pub const CHAIN_ID: &str = "columbus-5";
pub const LCD_URL: &str = "http://127.0.0.1:1317";
pub const MEKAPI_URL: &str = "http://127.0.0.1:4401";
pub const RPC_URL: &str = "ws://127.0.0.1:26657/websocket";

pub fn host() -> Option<String> {
    hostname::get()
        .ok()
        .and_then(|host| host.to_str().map(ToString::to_string))
}
