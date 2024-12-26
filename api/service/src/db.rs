use entity::checkpoint::{ActiveModel, Entity as Checkpoint, RpcCheckpointInfo};
use sea_orm::{
    prelude::Expr, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
/// Wrapper around the database connection
pub struct DatabaseWrapper {
    pub db: DatabaseConnection,
}

impl DatabaseWrapper {
    /// Create a new database wrapper with the given database URL
    pub async fn new(database_url: &str) -> Self {
        let db = Database::connect(database_url)
            .await
            .expect("Failed to connect to PostgreSQL");
        Self { db }
    }

    /// Insert a new checkpoint into the database
    pub async fn insert_checkpoint(&self, checkpoint: RpcCheckpointInfo) {
        let idx: i64 = checkpoint.idx.try_into().unwrap();
        let previous_idx: i64 = idx - 1;

        if previous_idx > 0 {
            let previous_checkpoint_exists = Checkpoint::find()
                .filter(entity::checkpoint::Column::Idx.eq(previous_idx))
                .one(&self.db)
                .await
                .map(|result| result.is_some())
                .unwrap_or(false);

            // checkpoints must be continuous, better to restart to re-sync from a valid checkpoint
            if !previous_checkpoint_exists {
                error!(
                    "Cannot insert checkpoint with idx {}: previous checkpoint with idx {} does not exist",
                    checkpoint.idx, previous_idx
                );
                return;
            }
        }

        // Insert the checkpoint
        let active_model: ActiveModel = checkpoint.into();
        match Checkpoint::insert(active_model).exec(&self.db).await {
            Ok(_) => info!("Checkpoint with idx {} inserted successfully", idx),
            Err(err) => error!("Error inserting checkpoint with idx {}: {:?}", idx, err),
        }
    }

    /// Fetch a checkpoint by its index
    pub async fn get_checkpoint_by_idx(&self, idx: u64) -> Option<RpcCheckpointInfo> {
        match Checkpoint::find()
            .filter(entity::checkpoint::Column::Idx.eq(idx as i64))
            .one(&self.db)
            .await
        {
            Ok(Some(checkpoint)) => {
                info!("Checkpoint found by idx: {:?}", idx);
                Some(checkpoint.into())
            }
            Ok(None) => None,
            Err(err) => {
                error!("Error fetching checkpoint by idx: {:?}", err);
                None
            }
        }
    }

    /// Fetch a checkpoint by its L2 block ID
    pub async fn get_checkpoint_by_l2_blockid(
        &self,
        l2_blockid: &str,
    ) -> Option<RpcCheckpointInfo> {
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

    // TODO: move this out of db and have a separate pagination wrapper module
    pub async fn get_paginated_checkpoints(
        &self,
        current_page: u64,
        page_size: u64,
        absolute_first_page: u64,
    ) -> PaginationInfo<RpcCheckpointInfo> {
        let total_checkpoints = self.get_total_checkpoint_count().await;
        let total_pages = (total_checkpoints as f64 / page_size as f64).ceil() as u64;
        let offset = (current_page - absolute_first_page) * page_size; // Adjust based on the first page

        // Convert `u64` to `i64` for compatibility with PostgreSQL
        let offset = offset.try_into().ok();
        let limit = page_size.try_into().ok();

        let items = match Checkpoint::find()
            .filter(Expr::col(entity::checkpoint::Column::Idx).is_not_null()) // Ensure idx is not NULL
            .order_by(entity::checkpoint::Column::Idx, sea_orm::Order::Asc) // Sort numerically
            .offset(offset)
            .limit(limit)
            .all(&self.db)
            .await
        {
            Ok(checkpoints) => checkpoints.into_iter().map(Into::into).collect(),
            Err(err) => {
                error!("Error fetching paginated checkpoints: {:?}", err);
                vec![]
            }
        };

        PaginationInfo {
            current_page,
            total_pages,
            absolute_first_page,
            items,
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

    /// Get the latest checkpoint index stored in the database
    pub async fn get_latest_checkpoint_index(&self) -> Option<i64> {
        use sea_orm::entity::prelude::*;

        match Checkpoint::find()
            .select_only()
            .column_as(entity::checkpoint::Column::Idx.max(), "max_idx")
            .into_tuple::<Option<i64>>() // Fetch the max value as a tuple
            .one(&self.db)
            .await
        {
            Ok(Some(max_idx)) => max_idx,
            Ok(_) => None, // If no checkpoints exist, return None
            Err(err) => {
                error!("Failed to fetch the latest checkpoint index: {:?}", err);
                None
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationInfo<T> {
    pub current_page: u64,
    pub total_pages: u64,
    pub absolute_first_page: u64, // Will be 0 or 1, depending on the context
    pub items: Vec<T>,            // The items for the current page
}
