#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[errror(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Cosmrs(#[from] cosmrs::ErrorReport),
    #[error(transparent)]
    TerraRustApi(#[from] terra_rust_api::errors::TerraRustAPIError),
}
