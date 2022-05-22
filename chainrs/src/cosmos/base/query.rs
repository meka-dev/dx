use serde::{Deserialize, Serialize};

use crate::json::serialize_field_as_string;

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct PageResponse {
    pub next_key: Option<Vec<u8>>,
    #[serde(with = "serialize_field_as_string")]
    pub total: u64,
}
