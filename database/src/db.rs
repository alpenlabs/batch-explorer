use model::{checkpoint::{ActiveModel, Entity as Checkpoint, RpcCheckpointInfo}, block::{RpcBlockHeader, ActiveModel as BlockActiveModel, Entity as Block}};
use sea_orm::{
    prelude::*, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use model::pgu64::PgU64;
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
                .filter(model::checkpoint::Column::Idx.eq(idx))
                .one(&self.db)
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
        match Checkpoint::insert(active_model).exec(&self.db).await {
            Ok(_) => info!("Checkpoint with idx {} inserted successfully", idx),
            Err(err) => error!("Error inserting checkpoint with idx {}: {:?}", idx, err),
        }
    }

    /// Fetch a checkpoint by its index
    pub async fn get_checkpoint_by_idx(&self, idx: i64) -> Option<RpcCheckpointInfo> {
        match Checkpoint::find()
            .filter(model::checkpoint::Column::Idx.eq(idx))
            .one(&self.db)
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
        block_height: i64,
    ) -> Result<Option<i64>, DbErr> {
        tracing::debug!("Searching for block with height: {}", block_height);
    
        match Block::find()
            .filter(model::block::Column::Height.eq(block_height))
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
            .filter(Expr::col(model::checkpoint::Column::Idx).is_not_null()) // Ensure idx is not NULL
            .order_by(model::checkpoint::Column::Idx, sea_orm::Order::Asc) // Sort numerically
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
            .column_as(model::checkpoint::Column::Idx.max(), "max_idx")
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
    pub async fn insert_block(&self, rpc_block_header: RpcBlockHeader, checkpoint_idx: i64)   {
        // Use `From` to convert `RpcBlockHeader` into an `ActiveModel`
        let mut active_model: BlockActiveModel = rpc_block_header.into();
 
        let height = active_model.height.clone().unwrap();
        let block_id = active_model.block_hash.clone().unwrap();

        // ensure that blocks exist incrementally and continuously
        // TODO: it should have been enforced by autoincrement constraint
        // TODO: move this logic to a wrapper
        let last_block = self.get_latest_block_index().await;
        if let Some(last_block_height) = last_block {
            if last_block_height  != height.checked_sub(1).unwrap()  {
                panic!("last_block_height does not match the expected height!"); 
            }
        }


        // TODO: remove this type conversion
        active_model.checkpoint_idx = Set(checkpoint_idx);

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
    /// Get the latest checkpoint index stored in the database
    pub async fn get_latest_block_index(&self) -> Option<i64> {
        // use sea_orm::entity::prelude::*;

        match Block::find()
            .select_only()
            .column_as(model::block::Column::Height.max(), "max_height")
            .into_tuple::<Option<i64>>() // Fetch the max value as a tuple
            .one(&self.db)
            .await
        {
            Ok(Some(max_height)) => max_height,
            Ok(_) => None, // If no checkpoints exist, return None
            Err(err) => {
                error!("Failed to fetch the latest  block index: {:?}", err);
                None
            }
        }
    }
    pub async fn _block_exists(&self, height: i64) -> bool {
        Block::find()
            .filter(model::block::Column::Height.eq(height))
            .one(&self.db)
            .await
            .map(|result| result.is_some())
            .unwrap_or(false)
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
