#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Cosmrs(#[from] cosmrs::ErrorReport),
    #[error(transparent)]
    TerraRustApi(#[from] terra_rust_api::errors::TerraRustAPIError),
}
