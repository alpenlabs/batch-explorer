pub mod api_service;
pub mod block_service;
pub mod checkpoint_service;

use serde::{Deserialize, Serialize};
// Struct for pagination parameters
#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub p: Option<u64>,
    pub ps: Option<u64>,
    pub error_msg: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchQuery {
    pub query: String,
}
