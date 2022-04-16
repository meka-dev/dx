use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unsupported")]
    Unsupported {},

    #[error("UnsupportedStartAsset")]
    UnsupportedStartAsset {},

    #[error("Insufficient")]
    Insufficient {},

    #[error("InsufficientPostBalance")]
    InsufficientPostBalance {},

    #[error("PathTooShort")]
    PathTooShort {},
}
