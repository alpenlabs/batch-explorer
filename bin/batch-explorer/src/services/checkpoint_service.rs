use fullnode_client::fetcher::StrataFetcher;
use tokio::sync::mpsc::Sender;
use database::db::DatabaseWrapper;
use entity::checkpoint::RpcCheckpointInfo;
use std::sync::Arc;
use std::cmp::min;
use tracing::info;

use crate::services::block_service::CheckpointFetch;

pub async fn start_checkpoint_fetcher(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    tx: Sender<CheckpointFetch>,
    fetch_interval: u64,
) {
    info!("Starting checkpoint fetcher...");
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(fetch_interval));

    loop {
        interval.tick().await;
        match fetch_checkpoints(fetcher.clone(), database.clone(), tx.clone()).await {
            Ok(_) => (),
            Err(e) => tracing::error!("Error fetching checkpoints: {}", e),
        }
    }
}

async fn fetch_checkpoints(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    tx: Sender<CheckpointFetch>,
) -> anyhow::Result<()> {
    info!("Fetching checkpoints from fullnode...");
    let fullnode_last_checkpoint = fetcher.get_latest_index("strata_getLatestCheckpointIndex").await?;
    let starting_checkpoint = get_starting_checkpoint_idx(database.clone()).await?;
    for idx in (starting_checkpoint as i64)..=fullnode_last_checkpoint {
        if !database.checkpoint_exists(idx).await{
            if let Ok(checkpoint) = fetcher.fetch_data::<RpcCheckpointInfo>("strata_getCheckpointInfo", idx).await {
                database.insert_checkpoint(checkpoint.clone()).await;
            }
        }
        let range = CheckpointFetch::new(idx); 
        tx.send(range).await?;
    }
    Ok(())
}


async fn get_starting_checkpoint_idx(db: Arc<DatabaseWrapper>) -> anyhow::Result<i32> {
    let last_block = db.get_latest_block_index().await;
    
    let local_last_checkpoint = db.get_latest_checkpoint_index().await.unwrap_or(-1);
    info!(last_block, local_last_checkpoint,  "determining starting checkpoint");
    // if we do not have a checkpoint in db start from 0
    if local_last_checkpoint == -1 {
        return Ok(0)
    }
    // we are calling it probable_* to consider some weirdest condition when 
    // we have the block but no any earlier checkpoint (before where block corresponds)
    let probable_starting_checkpoint: i32 = if let Some(block_height) = last_block {
        db.get_checkpoint_idx_by_block_height(block_height ).await?.unwrap_or_default()
    } else {
        0
    };

    Ok(min(probable_starting_checkpoint, local_last_checkpoint as i32))
}
