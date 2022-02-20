use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryContractStoreResponse<T> {
    pub query_result: T,
}
