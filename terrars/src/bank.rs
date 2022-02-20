use serde::{Deserialize, Serialize};

use crate::{base::query::PageResponse, Coin};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct QueryAllBalancesResponse {
    pub balances: Vec<Coin>,
    pub pagination: PageResponse,
}
