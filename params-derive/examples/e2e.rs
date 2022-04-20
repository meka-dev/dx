use std::sync::Arc;

use params_derive::params;
use tokio::sync::RwLock;

/// Such params, much joy
#[params]
pub struct Params {
    /// for curious eyes only.
    pub readable: String,

    /// change things dynamically at runtime.
    #[value(dynamic)]
    pub toggle: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = Params {
        readable: "foo".to_string(),
        toggle: Arc::new(RwLock::new(false)),
    };
    let (_params, app) = params.router();

    Ok(axum::Server::bind(&"127.0.0.1:9999".parse()?)
        .serve(app.into_make_service())
        .await?)
}
