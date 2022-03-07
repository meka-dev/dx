use std::env;

pub use tracing::{debug, error, info};

pub fn init() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}
