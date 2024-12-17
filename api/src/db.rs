//! Database module for managing checkpoint storage using RocksDB.
//! Provides functionality to store, retrieve, and paginate checkpoint information.

use crate::models::RpcCheckpointInfo;
use rocksdb::{Options, DB, IteratorMode};
use std::sync::Arc;
use tracing::info;

/// Database wrapper for RocksDB operations on checkpoints.
/// Handles serialization/deserialization of checkpoint data and provides
/// a high-level interface for checkpoint management.
pub struct Database {
    pub db: Arc<DB>,
}

impl Database {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = Arc::new(DB::open(&opts, path).expect("Failed to open RocksDB"));
        Self { db }
    }
    /// Inserts a checkpoint into the database.
    ///
    /// Uses big-endian byte ordering for numeric keys to maintain
    /// natural sorting order in RocksDB.
    ///
    /// # Parameters
    /// * `checkpoint` - The checkpoint information to store
    ///
    /// # Examples
    /// ```
    /// let db = CheckpointDB::new("path/to/db");
    /// db.insert_checkpoint(&checkpoint_info);
    /// ```
    pub fn insert_checkpoint(&self, checkpoint: &RpcCheckpointInfo) {
        // Use big-endian bytes for numeric key ordering
        let key = checkpoint.idx.to_be_bytes(); 
        let value = serde_json::to_vec(checkpoint).unwrap(); // Serialize to binary
        self.db.put(key, value).unwrap();
        info!("Checkpoint added: {:?}", checkpoint.idx);
    }


    /// Retrieves a checkpoint by its index.
    ///
    /// # Parameters
    /// * `idx` - The checkpoint index to retrieve
    ///
    /// # Returns
    /// * `Option<RpcCheckpointInfo>` - The checkpoint if found, None otherwise
    pub fn get_checkpoint(&self, idx: u64) -> Option<RpcCheckpointInfo> {
        let key = idx.to_string();
        match self.db.get(key).unwrap() {
            Some(value) => serde_json::from_slice(&value).ok(),
            None => None,
        }
    }

    /// Retrieves a paginated list of checkpoints.
    ///
    /// # Parameters
    /// * `offset` - Starting index for pagination
    /// * `limit` - Maximum number of checkpoints to return
    ///
    /// # Returns
    /// * `(Vec<RpcCheckpointInfo>, u64)` - Tuple containing:
    ///   - Vector of checkpoint information
    ///   - Total count of checkpoints in the database
    pub async fn get_paginated_checkpoints(
        &self,
        offset: u64,
        limit: u64,
    ) -> (Vec<RpcCheckpointInfo>, u64) {
        let checkpoints = self.db.iterator(IteratorMode::From(
            &offset.to_be_bytes(),
            rocksdb::Direction::Forward,
        ))
        .take(limit as usize)
        .map(|result| {
            let (key, value) = result.expect("Failed to iterate RocksDB");
            // Deserialize value toRpcCheckpointInfo 
            serde_json::from_slice::<RpcCheckpointInfo>(&value).unwrap()
        })
        .collect::<Vec<RpcCheckpointInfo>>();

        // Fetch the total count of checkpoints for pagination metadata
        let total_checkpoints = self.get_total_checkpoint_count();

        (checkpoints, total_checkpoints)
    }

    pub fn get_total_checkpoint_count(&self) -> u64 {
        match self.db.property_int_value("rocksdb.estimate-num-keys") {
            Ok(value) => value.unwrap_or(0), // If successful, return the value
            Err(_) => 0, // Handle failure gracefully, maybe return 0 or fallback to counting keys
        }
    }
}
