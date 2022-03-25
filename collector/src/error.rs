use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::error::TrySendError;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Error {
    EmptyPayload,
    InvalidNamespace { reason: String },
    InvalidHex { line_number: usize, reason: String },
    ParseLines { line_number: usize },
    SinkClosed,
    SinkFull,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::from(&self), Json(self)).into_response()
    }
}

impl From<&Error> for StatusCode {
    fn from(error: &Error) -> Self {
        match error {
            Error::EmptyPayload => StatusCode::BAD_REQUEST,
            Error::InvalidNamespace { .. }
            | Error::InvalidHex { .. }
            | Error::ParseLines { .. } => StatusCode::BAD_REQUEST,
            Error::SinkClosed => StatusCode::SERVICE_UNAVAILABLE,
            Error::SinkFull => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl<T> From<TrySendError<T>> for Error {
    fn from(input: TrySendError<T>) -> Self {
        match input {
            TrySendError::Closed(_) => Self::SinkClosed,
            TrySendError::Full(_) => Self::SinkFull,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use serde_json::json;

    use super::Error;

    #[test]
    fn error_json_roundtrip() {
        let value = json!({
            "kind": "parse_lines",
            "line_number": 12,
        });
        let error: Error = serde_json::from_value(value).unwrap();
        assert_matches!(error, Error::ParseLines { line_number } if line_number == 12);
    }
}
