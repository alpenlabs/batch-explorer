// services/api_service.rs
use super::QueryParams;
use super::SearchQuery;
use axum::{
    extract::{Query, State},
    Json,
};
use database::connection::DatabaseWrapper;
use database::services::checkpoint_service::CheckpointService;
use hex;
use model::pgu64::PgU64;
use serde_json::json;
use std::sync::Arc;
pub async fn checkpoints(
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<QueryParams>,
) -> Json<serde_json::Value> {
    let current_page = params.p.unwrap_or(1);
    let page_size = params.ps.unwrap_or(10);
    let error_msg = params.error_msg.clone();
    tracing::debug!("error_msg: {:?}", error_msg);

    let checkpoint_db = CheckpointService::new(&database.db);
    let paginated_data = checkpoint_db
        .get_paginated_checkpoints(current_page, page_size, 1, None) // Set absolute_first_page to 1 for checkpoint tables
        .await;
    Json(json!({ "result": paginated_data }))
}

pub async fn checkpoint(
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<QueryParams>,
) -> Json<serde_json::Value> {
    let current_page = params.p.unwrap_or(0); // Default to page 0
    let page_size = 1; // Set constant page size=1 for detail page

    let checkpoint_db = CheckpointService::new(&database.db);
    // Get paginated checkpoints
    let mut paginated_data = checkpoint_db
        .get_paginated_checkpoints(current_page, page_size, 0, Some("asc"))
        .await;
    paginated_data.total_pages -= 1; // Adjust total pages for 0-based indexing
    Json(json!({ "result": paginated_data }))
}

pub async fn search(
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<SearchQuery>,
) -> Json<serde_json::Value> {
    let mut query = params.query.trim();
    let checkpoint_db = CheckpointService::new(&database.db);

    // Check if it's a valid block number
    if let Ok(block_number) = query.parse::<u64>() {
        tracing::info!("Search request for block number: {}", block_number);
        let block_number = PgU64(block_number).to_i64();
        if let Ok(Some(checkpoint_idx)) = checkpoint_db
            .get_checkpoint_idx_by_block_height(block_number)
            .await
        {
            let checkpoint_idx = PgU64::from_i64(checkpoint_idx).0;
            return Json(json!({"result": checkpoint_idx}));
        }
    }

    // Remove the "0x" prefix if present
    if query.starts_with("0x") {
        query = query.trim_start_matches("0x");
    }

    // Check if the length is 64 characters (32 bytes)
    if query.len() == 64 {
        // Check if it's a valid hex string
        if hex::decode(query).is_ok() {
            tracing::info!("Search request for block hash: {}", query);
            if let Ok(Some(checkpoint_idx)) =
                checkpoint_db.get_checkpoint_idx_by_block_hash(query).await
            {
                let checkpoint_idx = PgU64::from_i64(checkpoint_idx).0;
                return Json(json!({"result": checkpoint_idx}));
            } else {
                tracing::info!("No checkpoint found for block hash: {}", query);
            }
        }
    }
    Json(json!({ "error": "Invalid search entry" }))
}
