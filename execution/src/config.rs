use std::net::SocketAddr;

pub const CHAIN_ID: &str = "columbus-5";
pub const LCD_URL: &str = "http://127.0.0.1:1317";
pub const RPC_URL: &str = "ws://127.0.0.1:26657/websocket";

pub const VERSION: &str = env!("HASH");

#[derive(argh::FromArgs, Debug)]
/// Execution environemnt for Mekatek strategies.
pub struct Args {
    /// terra chain identifier, should be static except on hard fork (Mainnet: columbus-5, Testnet:
    /// bombay-12)
    #[argh(option, default = "CHAIN_ID.to_string()")]
    pub chain_id: String,

    /// HTTP endpoint for Terra's LCD (http://127.0.0.1:1317)
    #[argh(option, default = "LCD_URL.to_string()")]
    pub lcd_url: String,

    /// HTTP server for metrics. (127.0.0.1:4444)
    #[argh(option, default = "([127, 0, 0, 1], 4444).into()")]
    pub metrics_addr: SocketAddr,

    /// WS endpoint for Terra's tendermint RPC (ws://127.0.0.1:26657/websocket)
    #[argh(option, default = "RPC_URL.to_string()")]
    pub rpc_url: String,
}
