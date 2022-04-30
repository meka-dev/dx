use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
};

use params_derive::params;
use tendermint::chain;
use terrars::AccountId;

const CHAIN_ID: &str = "columbus-5";
const LCD_URL: &str = "http://127.0.0.1:1317";
const MEKAPI_URL: &str = "http://127.0.0.1:4401";
const RPC_URL: &str = "ws://127.0.0.1:26657/websocket";

/// Execution environemnt for Mekatek strategies.
#[params]
#[derive(argh::FromArgs, Debug)]
pub struct Args {
    /// terra chain identifier, should be static except on hard fork (Mainnet: columbus-5, Testnet:
    /// bombay-12)
    #[argh(option, default = "CHAIN_ID.parse().unwrap()")]
    pub chain_id: chain::Id,

    /// signs and broadcasts tx to the network when disabled.
    #[argh(
        option,
        default = "Arc::new(RwLock::new(false))",
        from_str_fn(into_dynamic)
    )]
    #[value(dynamic)]
    pub execute_trades: bool,

    /// HTTP endpoint for Terra's LCD (http://127.0.0.1:1317)
    #[argh(option, default = "LCD_URL.to_string()")]
    pub lcd_url: String,

    /// HTTP endpoint for Terra's LCD (http://127.0.0.1:1317)
    #[argh(option, default = "MEKAPI_URL.to_string()")]
    pub mekapi_url: String,

    /// HTTP server for metrics. (127.0.0.1:4444)
    #[argh(option, default = "([127, 0, 0, 1], 4444).into()")]
    pub metrics_addr: SocketAddr,

    /// HTTP server for params reading and manipulation. (127.0.0.1:4445)
    #[argh(option, default = "([127, 0, 0, 1], 4445).into()")]
    pub params_addr: SocketAddr,

    /// WS endpoint for Terra's tendermint RPC (ws://127.0.0.1:26657/websocket)
    #[argh(option, default = "RPC_URL.to_string()")]
    pub rpc_url: String,

    /// location of the file on disk holding the secret key (./secret.key)
    #[argh(option, default = "String::from(\"./secret.key\")")]
    pub secret_key_file: String,

    /// address of contract that atomic arbitrage trades go through
    #[argh(
        option,
        default = "AccountId::from_str(\"terra1lerw0sk4zt4nkxqr465ekgflpl4e98kl3ydm58\").unwrap()"
    )]
    pub arby_addr: AccountId,

    /// address of treasury which pays for fees and on which behalf execute are issued
    #[argh(
        option,
        default = "AccountId::from_str(\"terra1sstgnc5rvmhte5rju9vh75wgnf3dktavmawzrp\").unwrap()"
    )]
    pub treasury: AccountId,
}

fn into_dynamic<T: FromStr>(input: &str) -> Result<Arc<RwLock<T>>, String> {
    let value = input
        .parse::<T>()
        .map_err(|_| format!("unable to parse value from input: {}", input))?;

    Ok(Arc::new(RwLock::new(value)))
}
