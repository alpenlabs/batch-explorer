use database::db::DatabaseWrapper;
use entity::block::RpcBlockHeader;
use fullnode_client::fetcher::StrataFetcher;
use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use tracing::info;

/// Event sent to block fetcher to request fetching of blocks for the checkpoint
#[derive(Debug, Clone)]
pub struct CheckpointFetch {
    pub idx: i64,
}
impl CheckpointFetch {
    pub fn new(idx: i64) -> Self {
        Self { 
            idx
        }
    }
}
pub async fn run_block_fetcher(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    mut rx: Receiver<CheckpointFetch>,
) {
    info!("Starting block fetcher...");
    while let Some(CheckpointFetch{idx}) = rx.recv().await {
        info!("Received checkpoint: {:?}", idx);
        fetch_blocks_in_checkpoint(fetcher.clone(), database.clone(), idx as i32).await;
    }
}

async fn fetch_blocks_in_checkpoint(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    checkpoint_idx: i32,
) {
    let checkpoint = database.get_checkpoint_by_idx(checkpoint_idx as u64).await;
    if let Some(c) = checkpoint {
        let mut  start = c.l2_range.0 as i32;
        let end = c.l2_range.1 as i32;
        
        // we will reach this point only when we are sure that we must fetch from particular
        // checkpoint. So having the heighest among the blocks must give us the shortcut 
        // to determine the most optimal starting point.
        let last_block = database.get_latest_block_index().await;
        if let Some(last_block_height) = last_block{
            if last_block_height > start {
                start = last_block_height + 1 ;
            } 
        }
        for block_height in start..=end {
                if let Ok(block_headers) = fetcher.fetch_data::<Vec<RpcBlockHeader>>("strata_getHeadersAtIdx", block_height as i64).await {
                    for block_header in block_headers {
                        database.insert_block(block_header.clone(), checkpoint_idx as i64).await;
                    }
                }
        }
    }
}