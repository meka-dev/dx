use common::{signals, tracing};
use eyre::{eyre, Result};
use futures::FutureExt as _;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let (shutdown_sender, _shutdown_receiver) = mpsc::channel::<()>(1);
    let mut signals_handle = tokio::spawn(signals::routine(shutdown_sender)).fuse();

    // Bootstrap venues to produce price feeds.
    // Load models.
    // Setup model watcher.
    // Configure one valuer per model.
    // Run Trader.

    loop {
        tokio::select! {
            join = &mut signals_handle => {
                join?.map_err(|err| eyre!("failed to handle signal: {}", err))?;
                break;
            },

            else => break
        }
    }

    Ok(())
}
