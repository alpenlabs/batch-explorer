use database::connection::DatabaseWrapper;
use model::block::RpcBlockHeader;
use fullnode_client::fetcher::StrataFetcher;
use tokio::sync::mpsc::Receiver;
use std::sync::Arc;
use tracing::info;
use model::pgu64::PgU64;
use database::services::{checkpoint_service::CheckpointService, block_service::BlockService};

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
        fetch_blocks_in_checkpoint(fetcher.clone(), database.clone(), idx).await;
    }
}

async fn fetch_blocks_in_checkpoint(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    checkpoint_idx: i64,
) {
    let checkpoint_db = CheckpointService::new(&database.db);
    let block_db = BlockService::new(&database.db);
    let checkpoint = checkpoint_db.get_checkpoint_by_idx(checkpoint_idx).await;
    if let Some(c) = checkpoint {
        let mut  start = c.l2_range.0;
        let end = c.l2_range.1;
        
        // we will reach this point only when we are sure that we must fetch from particular
        // checkpoint. So having the heighest among the blocks must give us the shortcut 
        // to determine the most optimal starting point.
        let last_block = block_db.get_latest_block_index().await;
        if let Some(last_block_height) = last_block{
            let last_block_height_u64 =   PgU64::from_i64(last_block_height).0;
            // start from the next block
            if last_block_height_u64 >= start {
                start = last_block_height_u64 + 1 ;
            } 
        }
        info!("Fetching blocks from {} to {} for checkpoint {}", start, end, checkpoint_idx);
        for block_height in start..=end {
                if let Ok(block_headers) = fetcher.fetch_data::<Vec<RpcBlockHeader>>("strata_getHeadersAtIdx", block_height).await {
                    for block_header in block_headers {
                        block_db.insert_block(block_header.clone(), checkpoint_idx).await;
                    }
                }
        }
    }
}