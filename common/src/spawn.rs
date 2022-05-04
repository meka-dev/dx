use anyhow::Result;
use tokio::task::JoinHandle;

pub async fn flatten<T>(handle: JoinHandle<Result<T>>) -> Result<T> {
    match handle.await {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(anyhow::anyhow!("handle failed: {}", err)),
    }
}
