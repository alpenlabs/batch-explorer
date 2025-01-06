use model::{block::{RpcBlockHeader, ActiveModel as BlockActiveModel, Entity as Block}};
use sea_orm::{ ColumnTrait, DatabaseConnection,  EntityTrait, QueryFilter, QuerySelect, Set};
use tracing::error;
/// Wrapper around the database connection
pub struct BlockService<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> BlockService<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
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
        match Block::insert(active_model).exec(self.db).await {
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
            .one(self.db)
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
            .one(self.db)
            .await
            .map(|result| result.is_some())
            .unwrap_or(false)
    }
}

