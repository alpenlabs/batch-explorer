use model::{checkpoint::{ActiveModel, Entity as Checkpoint, RpcCheckpointInfo}, block:: Entity as Block};
use sea_orm::{
    prelude::*, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Order,
    QuerySelect, Set
};
use tracing::{error, info};
use model::pgu64::PgU64;
use crate::services::pagination::PaginatedData;
use super::utils::resolve_order;
pub struct CheckpointService<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> CheckpointService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
    pub async fn checkpoint_exists(&self, idx: i64) -> bool {
        Checkpoint::find()
                .filter(model::checkpoint::Column::Idx.eq(idx))
                .one(self.db)
                .await
                .map(|result| result.is_some())
                .unwrap_or(false)
    }
    /// Insert a new checkpoint into the database
    pub async fn insert_checkpoint(&self, checkpoint: RpcCheckpointInfo) {
        let idx: i64 = PgU64(checkpoint.idx).to_i64();

        if let Some(previous_idx) = idx.checked_sub(1){
            let previous_checkpoint_exists = self.checkpoint_exists(previous_idx).await;

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
        match Checkpoint::insert(active_model).exec(self.db).await {
            Ok(_) => info!("Checkpoint with idx {} inserted successfully", idx),
            Err(err) => error!("Error inserting checkpoint with idx {}: {:?}", idx, err),
        }
    }

    /// Fetch a checkpoint by its index
    pub async fn get_checkpoint_by_idx(&self, idx: i64) -> Option<RpcCheckpointInfo> {
        match Checkpoint::find()
            .filter(model::checkpoint::Column::Idx.eq(idx))
            .one(self.db)
            .await
        {
            Ok(Some(checkpoint)) => {
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
    pub async fn get_checkpoint_idx_by_block_hash(
        &self,
        block_hash: &str,
    ) -> Result<Option<i64>, DbErr> {
    
        match Block::find()
            .filter(model::block::Column::BlockHash.eq(block_hash))
            .one(self.db)
            .await
        {
            Ok(Some(block))=>{
                tracing::info!("Block found: {:?}", block);
                Ok(Some(block.checkpoint_idx))
            }
            Ok(None) => {
                tracing::info!("No block found for hash: {}", block_hash);
                Ok(None)
            }
            Err(err) => {
                tracing::error!("Query failed: {:?}", err);
                Err(err)
            }
        }
    }
    /// Fetch a checkpoint by its L2 block height
    pub async fn get_checkpoint_idx_by_block_height(
        &self,
        block_height: i64,
    ) -> Result<Option<i64>, DbErr> {
        tracing::debug!("Searching for block with height: {}", block_height);
    
        match Block::find()
            .filter(model::block::Column::Height.eq(block_height))
            .one(self.db)
            .await
        {
            Ok(Some(block)) => {
                tracing::info!("Block found: {:?}", block);
                Ok(Some(block.checkpoint_idx))
            }
            Ok(None) => {
                tracing::info!("No block found for height: {}", block_height);
                Ok(None)
            }
            Err(err) => {
                tracing::error!("Query failed: {:?}", err);
                Err(err)
            }
        }
    }
    // TODO: move this out of db and have a separate pagination wrapper module
    pub async fn get_paginated_checkpoints(
        &self,
        current_page: u64,
        page_size: u64,
        absolute_first_page: u64,
        order: Option<&str>
    ) -> PaginatedData<RpcCheckpointInfo> {
        let total_checkpoints = self.get_total_checkpoint_count().await;
        let total_pages = (total_checkpoints as f64 / page_size as f64).ceil() as u64;
        let offset = (current_page - absolute_first_page) * page_size; // Adjust based on the first page
        let order = resolve_order(order);
        // Convert `u64` to `i64` for compatibility with PostgreSQL
        let offset = offset.try_into().ok();
        let limit = page_size.try_into().ok();

        let items = match Checkpoint::find()
            .filter(Expr::col(model::checkpoint::Column::Idx).is_not_null()) // Ensure idx is not NULL
            .order_by(model::checkpoint::Column::Idx, order) // Sort numerically
            .offset(offset)
            .limit(limit)
            .all(self.db)
            .await
        {
            Ok(checkpoints) => checkpoints.into_iter().map(Into::into).collect(),
            Err(err) => {
                error!("Error fetching paginated checkpoints: {:?}", err);
                vec![]
            }
        };

        PaginatedData {
            current_page,
            total_pages,
            absolute_first_page,
            items,
        }
    }

    /// Get the total count of checkpoints in the database
    pub async fn get_total_checkpoint_count(&self) -> u64 {
        use sea_orm::entity::prelude::*;

        match Checkpoint::find().count(self.db).await {
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
            .column_as(model::checkpoint::Column::Idx.max(), "max_idx")
            .into_tuple::<Option<i64>>() // Fetch the max value as a tuple
            .one(self.db)
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

    /// Get the earliest checkpoint index whose status is either `Pending` or `Confirmed` or `-`
    pub async fn get_earliest_unfinalized_checkpoint_idx(&self) -> Option<i64> {
        match Checkpoint::find()
            .filter(
                model::checkpoint::Column::Status.eq("Pending")
                    .or(model::checkpoint::Column::Status.eq("Confirmed"))
                    .or(model::checkpoint::Column::Status.eq("-")),
            )
            .order_by(model::checkpoint::Column::Idx, Order::Asc)
            .one(self.db)
            .await
        {
            Ok(Some(checkpoint)) => Some(checkpoint.idx),
            Ok(None) => None,
            Err(err) => {
                error!("Error fetching earliest unfinalized checkpoint: {:?}", err);
                None
            }
        }
    }

    /// Update the status of a checkpoint
    pub async fn update_checkpoint_status(&self, checkpoint_idx: i64, status: String) -> Result<(), DbErr> {
        match Checkpoint::find()
            .filter(model::checkpoint::Column::Idx.eq(checkpoint_idx))
            .one(self.db)
            .await
        {
            Ok(Some(checkpoint)) => {
                let mut active_model: ActiveModel = checkpoint.into();
                active_model.status = Set(status.to_string());

                match active_model.update(self.db).await {
                    Ok(_) => {
                        info!(
                            "Checkpoint with idx {} updated successfully to status: {}",
                            checkpoint_idx, status
                        );
                        Ok(())
                    },
                    Err(err) => {
                        error!(
                            "Failed to update checkpoint with idx {}: {:?}",
                            checkpoint_idx, err
                        );
                        Err(err)
                    },
                }
            }
            Ok(None) => {
                error!("Checkpoint with idx {} not found", checkpoint_idx);
                Err(DbErr::RecordNotFound(format!("Checkpoint with idx {} not found", checkpoint_idx)))
            }
            Err(err) => {
                error!(
                    "Error querying checkpoint with idx {}: {:?}",
                    checkpoint_idx, err
                );
                Err(err)
            }
        }
    }
}
