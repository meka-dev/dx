use anyhow::{Context as _, Result};

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

pub fn host() -> Result<String> {
    Ok(hostname::get()
        .context("failed to read hostname")?
        .to_str()
        .expect("hostname not present")
        .to_owned())
}
