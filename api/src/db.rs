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

        let key_l2_blockid = &checkpoint.l2_blockid;
        self.db.put(key_l2_blockid, key).unwrap();
        info!("Checkpoint added: {:?}", checkpoint.idx);
    }


    /// Retrieves a checkpoint by its index.
    ///
    /// # Parameters
    /// * `idx` - The checkpoint index to retrieve
    ///
    /// # Returns
    /// * `Option<RpcCheckpointInfo>` - The checkpoint if found, None otherwise
    pub fn get_checkpoint_by_idx(&self, idx: u64) -> Option<RpcCheckpointInfo> {
        let key = idx.to_be_bytes();
        match self.db.get(key).unwrap() {
            Some(value) => serde_json::from_slice(&value).ok(),
            None => None,
        }
    }

    pub fn get_checkpoint_by_l2_blockid(&self, l2_blockid: &str) -> Option<RpcCheckpointInfo> {
        if let Ok(Some(value_idx)) = self.db.get(l2_blockid.as_bytes()) {
            let idx = u64::from_be_bytes(value_idx.try_into().unwrap());
            return self.get_checkpoint_by_idx(idx);
        }
        None
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

    /// Searches for a checkpoint by exact match of either index or L2 block ID.
    ///
    /// First attempts to parse the query as a checkpoint index.
    /// If that fails, tries to match it as an L2 block ID.
    ///
    /// # Parameters
    /// * `query` - String containing either checkpoint index or L2 block ID
    ///
    /// # Returns
    /// * `Option<RpcCheckpointInfo>` - Matching checkpoint if found, None otherwise
    ///
    /// # Examples
    /// ```
    /// // Search by index
    /// let result = db.search_exact_match("123");
    ///
    /// // Search by L2 block ID
    /// let result = db.search_exact_match("0x1234...");
    /// ```
    pub fn search_exact_match(&self, query: &str) -> Option<RpcCheckpointInfo> {
        // Attempt to parse the query as `idx`
        if let Ok(idx) = query.parse::<u64>() {
            info!("Search by checkpoint idx: {}", idx);
            if let Some(checkpoint) = self.get_checkpoint_by_idx(idx) {
                return Some(checkpoint);
            }
        }
        info!("Search by l2_blockid: {}", query);
        // If not idx, assume it's an l2_blockid
        if let Some(checkpoint) = self.get_checkpoint_by_l2_blockid(query) {
            return Some(checkpoint);
        }
        // If neither found
        None
    }
}
