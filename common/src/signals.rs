use anyhow::Result;
use tokio::{select, sync::mpsc};
use tracing::instrument;

use crate::tracing::info;

#[cfg(unix)]
#[instrument(name = "signals routine", skip(shutdown_tx))]
pub async fn routine(shutdown_tx: mpsc::Sender<()>) -> Result<()> {
    use tokio::signal::unix::*;

    let mut int = signal(SignalKind::interrupt())?;
    let mut quit = signal(SignalKind::quit())?;
    let mut term = signal(SignalKind::terminate())?;

    let signal = select! {
        _ = int.recv() => SignalKind::interrupt(),
        _ = quit.recv() => SignalKind::quit(),
        _ = term.recv() => SignalKind::terminate(),
    };

    info!(?signal, "received termination signal");
    let _ = shutdown_tx.try_send(());

    anyhow::Ok(())
}
