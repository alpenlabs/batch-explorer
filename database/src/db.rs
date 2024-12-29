use entity::{checkpoint::{ActiveModel, Entity as Checkpoint, RpcCheckpointInfo}, block::{RpcBlockHeader, ActiveModel as BlockActiveModel, Entity as Block}};
use sea_orm::{
    prelude::*, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set
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
            .expect(&format!("Failed to connect to PostgreSQL {}", database_url));
        Self { db }
    }
    pub async fn checkpoint_exists(&self, idx: i64) -> bool {
        Checkpoint::find()
                .filter(entity::checkpoint::Column::Idx.eq(idx))
                .one(&self.db)
                .await
                .map(|result| result.is_some())
                .unwrap_or(false)
    }
    /// Insert a new checkpoint into the database
    pub async fn insert_checkpoint(&self, checkpoint: RpcCheckpointInfo) {
        let idx: i64 = checkpoint.idx.try_into().unwrap();
        let previous_idx: i64 = idx - 1;

        if previous_idx > 0 {
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
    pub async fn get_checkpoint_idx_by_block_hash(
        &self,
        block_hash: &str,
    ) -> Result<Option<i32>, DbErr> {
    
        match Block::find()
            .filter(entity::block::Column::BlockHash.eq(block_hash))
            .one(&self.db)
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
        block_height: i32,
    ) -> Result<Option<i32>, DbErr> {
        tracing::debug!("Searching for block with height: {}", block_height);
    
        match Block::find()
            .filter(entity::block::Column::Height.eq(block_height))
            .one(&self.db)
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

    // /// Perform an exact match search on checkpoints
    // pub async fn search_exact_match(&self, query: &str) -> Option<RpcCheckpointInfo> {
    //     if let Ok(idx) = query.parse::<u64>() {
    //         return self.get_checkpoint_by_idx(idx).await;
    //     }
    //     self.get_checkpoint_by_l2_blockid(query).await
    // }

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
        /// Inserts a new block into the database
    /// Inserts a new block into the database, taking an `RpcBlockHeader` as input
    pub async fn insert_block(&self, rpc_block_header: RpcBlockHeader, checkpoint_idx: i64) {
        let height = rpc_block_header.block_idx as i64;
        let block_id = rpc_block_header.block_id.clone();

        // Ensure the block's checkpoint exists in the database
        let checkpoint_exists = self.checkpoint_exists(checkpoint_idx).await;

        if !checkpoint_exists {
            tracing::error!(
                "Cannot insert block with height {}: associated checkpoint with idx {} does not exist",
                height, checkpoint_idx
            );
            return;
        }

        // Use `From` to convert `RpcBlockHeader` into an `ActiveModel`
        let mut active_model: BlockActiveModel = rpc_block_header.into();

        // TODO: remove this
        let temp = checkpoint_idx as i32;
        active_model.checkpoint_idx = Set(temp);

        // Insert the block using the Entity::insert() method
        match Block::insert(active_model).exec(&self.db).await {
            Ok(_) => {
                tracing::info!(
                    "Block inserted successfully: height={}, block_hash={}",
                    height,
                    hex::encode(block_id)
                );
            }
            Err(err) => {
                tracing::error!(
                    "Error inserting block with height {}: {:?}",
                    height, err
                );
            }
        }
    }
    }

// TODO: keep it in separate pagination module
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationInfo<T> {
    pub current_page: u64,
    pub total_pages: u64,
    pub absolute_first_page: u64, // Will be 0 or 1, depending on the context
    pub items: Vec<T>,            // The items for the current page
}
