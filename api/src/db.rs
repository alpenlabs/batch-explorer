use crate::models::RpcCheckpointInfo;
use rocksdb::{Options, DB};
use std::sync::Arc;
use tracing::info;
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

    pub fn insert_checkpoint(&self, checkpoint: &RpcCheckpointInfo) {
        let key = checkpoint.idx.to_string();
        let value = serde_json::to_string(checkpoint).unwrap();
        self.db.put(key, value).unwrap();
        info!("Checkpoint added: {:?}", checkpoint.idx);
    }

    pub fn get_checkpoint(&self, idx: u64) -> Option<RpcCheckpointInfo> {
        let key = idx.to_string();
        match self.db.get(key).unwrap() {
            Some(value) => serde_json::from_slice(&value).ok(),
            None => None,
        }
    }
}
