use fullnode_client::fetcher::StrataFetcher;
use tokio::sync::mpsc::Sender;
use database::db::DatabaseWrapper;
use entity::checkpoint::RpcCheckpointInfo;
use std::sync::Arc;
use tracing::info;

use crate::services::block_service::CheckpointRange;

pub async fn start_checkpoint_fetcher(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    tx: Sender<CheckpointRange>,
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
    tx: Sender<CheckpointRange>,
) -> anyhow::Result<()> {
    info!("Fetching checkpoints from fullnode...");
    let fullnode_last_checkpoint = fetcher.get_latest_index("strata_getLatestCheckpointIndex").await?;
    let local_last_checkpoint = database.get_latest_checkpoint_index().await.unwrap_or(-1);

    for idx in (local_last_checkpoint + 1)..=fullnode_last_checkpoint as i64 {
        if let Ok(checkpoint) = fetcher.fetch_data::<RpcCheckpointInfo>("strata_getCheckpointInfo", idx).await {
            database.insert_checkpoint(checkpoint.clone()).await;
            let range = CheckpointRange {
                idx,
                start: checkpoint.l2_range.0 as i64,
                end: checkpoint.l2_range.1 as i64,
            };
            tx.send(range).await?;
        }
    }

    Ok(())
}