use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QuerySelect, PaginatorTrait};
use tracing::{info, error};
use entity::checkpoint::{Entity as Checkpoint, RpcCheckpointInfo, ActiveModel};

/// Wrapper around the database connection
pub struct DatabaseWrapper {
    pub db: DatabaseConnection,
}

impl DatabaseWrapper {
    /// Create a new database wrapper with the given database URL
    pub async fn new(database_url: &str) -> Self {
        let db = Database::connect(database_url).await.expect("Failed to connect to PostgreSQL");
        Self { db }
    }

    /// Insert a new checkpoint into the database
    pub async fn insert_checkpoint(&self, checkpoint: RpcCheckpointInfo) {
        let active_model: ActiveModel = checkpoint.into();
        match Checkpoint::insert(active_model).exec(&self.db).await {
            Ok(_) => info!("Checkpoint inserted successfully"),
            Err(err) => error!("Error inserting checkpoint: {:?}", err),
        }
    }

    /// Fetch a checkpoint by its index
    pub async fn get_checkpoint_by_idx(&self, idx: u64) -> Option<RpcCheckpointInfo> {
        match Checkpoint::find()
            .filter(entity::checkpoint::Column::Idx.eq(idx))
            .one(&self.db)
            .await
        {
            Ok(Some(checkpoint)) => Some(checkpoint.into()),
            Ok(None) => None,
            Err(err) => {
                error!("Error fetching checkpoint by idx: {:?}", err);
                None
            }
        }
    }

    /// Fetch a checkpoint by its L2 block ID
    pub async fn get_checkpoint_by_l2_blockid(&self, l2_blockid: &str) -> Option<RpcCheckpointInfo> {
        match Checkpoint::find()
            .filter(entity::checkpoint::Column::L2BlockId.eq(l2_blockid))
            .one(&self.db)
            .await
        {
            Ok(Some(checkpoint)) => Some(checkpoint.into()),
            Ok(None) => None,
            Err(err) => {
                error!("Error fetching checkpoint by L2 block ID: {:?}", err);
                None
            }
        }
    }

    /// Fetch a paginated list of checkpoints
/// Fetch a paginated list of checkpoints
pub async fn get_paginated_checkpoints(
    &self,
    offset: u64,
    limit: u64,
) -> Vec<RpcCheckpointInfo> {
    // Convert `u64` to `i64` for compatibility with PostgreSQL
    let offset = offset.try_into().ok(); // Convert `u64` to `Option<i64>`
    let limit = limit.try_into().ok(); // Convert `u64` to `Option<i64>`

    match Checkpoint::find()
        .offset(offset) // Use the converted `Option<i64>`
        .limit(limit)   // Use the converted `Option<i64>`
        .all(&self.db)
        .await
    {
        Ok(checkpoints) => checkpoints
            .into_iter()
            .map(Into::into)
            .collect(),
        Err(err) => {
            error!("Error fetching paginated checkpoints: {:?}", err);
            vec![]
        }
    }
}

    /// Perform an exact match search on checkpoints
    pub async fn search_exact_match(&self, query: &str) -> Option<RpcCheckpointInfo> {
        if let Ok(idx) = query.parse::<u64>() {
            return self.get_checkpoint_by_idx(idx).await;
        }
        self.get_checkpoint_by_l2_blockid(query).await
    }

    /// Get the total count of checkpoints in the database
    pub async fn get_total_checkpoint_count(&self) -> u64 {
        use sea_orm::entity::prelude::*;

        match Checkpoint::find().count(&self.db).await {
            Ok(count) => count,
            Err(err) => {
                error!("Failed to count checkpoints: {:?}", err);
                0
            }
        }
    }
}