use tokio::task::JoinHandle;

use crate::{err, Result};

pub async fn flatten<T>(handle: JoinHandle<Result<T>>) -> Result<T> {
    match handle.await {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(err!("handle failed: {}", err)),
    }
}
