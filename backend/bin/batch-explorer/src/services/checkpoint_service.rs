use fullnode_client::fetcher::StrataFetcher;
use tokio::sync::mpsc::Sender;
use database::connection::DatabaseWrapper;
use model::checkpoint::RpcCheckpointInfo;
use std::sync::Arc;
use std::cmp::min;
use tracing::{info, error, warn};
use model::pgu64::PgU64;
use crate::services::block_service::CheckpointFetch;
use database::services::{checkpoint_service::CheckpointService, block_service::BlockService};


/// This function fetches the checkpoints from the fullnode and inserts them into the database
/// It will run in a loop with a delay of `fetch_interval` seconds
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

/// This function fetches the checkpoints from the fullnode and inserts them into the database
/// It then sends the checkpoint index to the block fetcher task to fetch the corresponding block
async fn fetch_checkpoints(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    tx: Sender<CheckpointFetch>,
) -> anyhow::Result<()> {
    let checkpoint_db = CheckpointService::new(&database.db);
    info!("Fetching checkpoints from fullnode...");
    let fullnode_last_checkpoint = fetcher.get_latest_index("strata_getLatestCheckpointIndex").await?;
    let fn_chkpt_i64 = PgU64(fullnode_last_checkpoint).to_i64();
    let starting_checkpoint = get_starting_checkpoint_idx(database.clone()).await?;
    info!(fn_chkpt_i64, starting_checkpoint, "fetching checkpoints");
    for idx in (starting_checkpoint)..=fn_chkpt_i64 {
        if !checkpoint_db.checkpoint_exists(idx).await{
            let i = PgU64::from_i64(idx).0;
            if let Ok(checkpoint) = fetcher.fetch_data::<RpcCheckpointInfo>("strata_getCheckpointInfo", i).await {
                checkpoint_db.insert_checkpoint(checkpoint.clone()).await;
            }
        }
        let range = CheckpointFetch::new(idx); 
        tx.send(range).await?;
    }
    Ok(())
}

/// It is a helper function that returns the starting checkpoint index to start fetching from
/// It will return the minimum of the last checkpoint in the database and the checkpoint correcpoinding to 
/// last block in the database
async fn get_starting_checkpoint_idx(db: Arc<DatabaseWrapper>) -> anyhow::Result<i64> {
    let checkpoint_db = CheckpointService::new(&db.db);
    let block_db = BlockService::new(&db.db);

    let last_block = block_db.get_latest_block_index().await;
    
    let local_last_checkpoint = checkpoint_db.get_latest_checkpoint_index().await.unwrap_or(-1);
    // if we do not have a checkpoint in db start from 0
    if local_last_checkpoint == -1 {
        return Ok(i64::MIN)
    }
    // we are calling it probable_* to consider some weirdest condition when 
    // we have the block but no any earlier checkpoint (before where block corresponds)
    let probable_starting_checkpoint: i64 = if let Some(block_height) = last_block {
        checkpoint_db.get_checkpoint_idx_by_block_height(block_height ).await?.unwrap_or_default()
    } else {
        i64::MIN
    };

    Ok(min(probable_starting_checkpoint, local_last_checkpoint ))
}


/// This function starts the checkpoint status updater task
pub async fn start_checkpoint_status_updater_task(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    update_interval: u64,
) {
    info!("Starting checkpoint status updater...");
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(update_interval));

    loop {
        interval.tick().await;
        match update_checkpoints_status(fetcher.clone(), database.clone()).await {
            Ok(_) => (),
            Err(e) => tracing::error!("Error fetching checkpoints: {}", e),
        }
    }
}

/// This function continuously updates the status of the checkpoints which are yet to be finalized.
/// 
/// This algorith works on the assumptions that the checkpoints must get finalized in the order they are created.
/// or fullnode must have finalized the checkpoint before the next one is finalized.
/// 
/// ** Algorithm **
/// 1. Get the earliest checkpoint idx whose status is Either pending or Confirmed
/// 2. Fetch the checkpoint from fullnode
/// 3. If the checkpoint status is different from the one in the database, 
///    a. update the status may be 
///    b. increment the idx and go to step 2
/// 4. Else break the loop
async fn update_checkpoints_status(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
) -> anyhow::Result<()> {
    let checkpoint_db = CheckpointService::new(&database.db);
    let mut idx = checkpoint_db.get_earliest_unfinalized_checkpoint_idx().await.ok_or_else(|| {
        info!("No unfinalized checkpoints found.");
        anyhow::anyhow!("No unfinalized checkpoints")
    })?;

    loop {
        let i = PgU64::from_i64(idx).0;
        let checkpoint_from_rpc = fetcher
            .fetch_data::<RpcCheckpointInfo>("strata_getCheckpointInfo", i)
            .await
            .map_err(|_| {
                error!("Error fetching checkpoint from fullnode");
                anyhow::anyhow!("Error fetching checkpoint from fullnode")
            })?;

        let checkpoint_in_db = checkpoint_db.get_checkpoint_by_idx(idx).await.ok_or_else(|| {
            info!("No checkpoint found in database for idx {}", idx);
            anyhow::anyhow!("No checkpoint in database")
        })?;

        let status = match checkpoint_from_rpc.confirmation_status {
            Some(status) => status.to_string(),
            None => {
                warn!("Checkpoint status is None for idx {}", idx);
                return Ok(()); // Simply return and continue execution instead of erroring  
            }
        };
        info!("Updating checkpoint status: idx={}, status={}", idx, status.clone());
        // if there is no change in status, return by doing nothing
        if checkpoint_in_db
        .confirmation_status
        .map_or("-".to_string(), |s| s.to_string()) == status.to_string() 
        {
            return Ok(());
        }
        
        checkpoint_db
        .update_checkpoint_status(idx, status.to_string())
        .await
        .map_err(|e| {
            error!("Error updating checkpoint status: {:?}", e);
                anyhow::anyhow!("Failed to update checkpoint status")
            })?;

        idx = idx.saturating_add(1);
    }
}