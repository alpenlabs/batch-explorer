use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{db::Database, fetcher::StrataFetcher, models::RpcCheckpointInfo};
// use crate::models::Batch;
use std::sync::Arc;
use serde::Serialize;
use tracing::{info, error};

#[derive(Serialize)]
#[serde(untagged)] // Removes the enum tag in the serialized JSON
pub enum CheckpointResponse {
    Success(RpcCheckpointInfo),
    Error { error: String },
}


pub async fn fetch_and_store_checkpoint(
    State((db, fetcher)): State<(Arc<Database>, Arc<StrataFetcher>)>,
    Path(idx): Path<u64>,
) -> impl IntoResponse {
    match fetcher.fetch_checkpoint(idx).await {
        Ok(checkpoint) => {
            db.insert_checkpoint(&checkpoint);
            info!("Checkpoint added: {:?}", checkpoint.idx);
            (
                StatusCode::CREATED,
                Json(CheckpointResponse::Success(checkpoint)),
            )
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR, // Propagate server error status
            Json(CheckpointResponse::Error {
                error: format!("Failed to fetch checkpoint: {}", err),
            }),
        ),
    }
}

/// get a checkpoint by its index
pub async fn get_checkpoint(
    State(db): State<Arc<Database>>,
    Path(idx): Path<u64>,
) -> impl IntoResponse {
    if let Some(checkpoint) = db.get_checkpoint(idx) {
        return (
            StatusCode::CREATED,
            Json(CheckpointResponse::Success(checkpoint)),
        );
    }
    (
        StatusCode::NOT_FOUND,
        Json(CheckpointResponse::Error {
            error: "Checkpoint not found".to_string(),
        }),
    )
}

use serde_json::{json, Value};
/// Temporary struct for 
#[derive(Serialize)]
pub struct CheckpointResponseTemp {
    pub message: String,
    pub data: Option<Value>,  // Holds either success data or error details
}

// TODO: remove this method in production. Its utility is only for testing purposes
// and should not be exposed in a production environment.

/// Generate arbitrary sample data for testing purposes.
/// It uses checkpoint 0 as a reference point and replicates it n times with
/// increasing `idx`
pub async fn generate_sample_data(
    State((db, fetcher)): State<(Arc<Database>, Arc<StrataFetcher>)>,
    Path(start_idx): Path<u64>,
) -> impl IntoResponse {
    let mut inserted_checkpoints = Vec::new();
    
    info!("To start from checkpoint: {:?}", start_idx);
    // Fetch a checkpoint from the fullnode
    match fetcher.fetch_checkpoint(start_idx).await {
        Ok(original_checkpoint) => {
            // Replicate the checkpoint data 500 times with increasing `idx`
            for i in 0..1000000 {
                let mut new_checkpoint = original_checkpoint.clone();
                new_checkpoint.idx = start_idx + i; // Set the new idx for each replica

                // Insert into database
                db.insert_checkpoint(&new_checkpoint);
                
                // Add to the list of inserted checkpoints (for response)
                inserted_checkpoints.push(new_checkpoint.idx);
            }

            // Return the inserted checkpoints as confirmation
            let response = CheckpointResponseTemp {
                message: "Checkpoints successfully added".to_string(),
                data: Some(json!(inserted_checkpoints)),
            };

            (StatusCode::CREATED, Json(response))
        }
        Err(err) => {
            error!("Failed to fetch checkpoint: {}", err);
            // If there is an error fetching the checkpoint
            let response = CheckpointResponseTemp {
                message: "Failed to fetch checkpoint".to_string(),
                data: Some(json!(format!("{}", err))),
            };

            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}