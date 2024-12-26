use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, QuerySelect, QueryOrder, prelude::Expr};
use tracing::{info, error};
use entity::checkpoint::{Entity as Checkpoint, RpcCheckpointInfo, ActiveModel};
use serde::{Serialize, Deserialize};
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
        info!("teest---------");
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationInfo<T> {
    pub current_page: u64,
    pub total_pages: u64,
    pub absolute_first_page: u64, // Will be 0 or 1, depending on the context
    pub items: Vec<T>,           // The items for the current page
}