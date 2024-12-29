use database::db::DatabaseWrapper;
use entity::block::RpcBlockHeader;
use fullnode_client::fetcher::StrataFetcher;
use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use tracing::info;
use entity::checkpoint::RpcCheckpointInfo;
#[derive(Debug, Clone)]
pub struct CheckpointRange {
    pub idx: i64,
    pub start: i64,
    pub end: i64,
}
impl CheckpointRange {
    pub fn new(checkpoint: RpcCheckpointInfo ) -> Self {
        Self { 
            idx: checkpoint.idx as i64,
            start: checkpoint.l2_range.0 as i64,
            end: checkpoint.l2_range.1 as i64,
        }
    }
}
pub async fn run_block_fetcher(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    mut rx: Receiver<CheckpointRange>,
) {
    info!("Starting block fetcher...");
    while let Some(range) = rx.recv().await {
        info!("Received checkpoint range: {:?}", range);
        fetch_blocks_in_range(fetcher.clone(), database.clone(), range).await;
    }
}

async fn fetch_blocks_in_range(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    range: CheckpointRange,
) {
    for block_height in range.start..=range.end {
        if let Ok(block_headers) = fetcher.fetch_data::<Vec<RpcBlockHeader>>("strata_getHeadersAtIdx", block_height).await {
            for block_header in block_headers {
                database.insert_block(block_header.clone(), range.idx).await;
            }
        }
    }
}