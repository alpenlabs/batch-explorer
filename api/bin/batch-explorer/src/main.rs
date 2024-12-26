mod helper;

use fullnode_client::fetcher::StrataFetcher;
use serde::Deserialize;
use service::db::DatabaseWrapper;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tokio::sync::mpsc::{self, Receiver, Sender};
use entity::{checkpoint::RpcCheckpointInfo, block::RpcBlockHeader};
use sea_orm::Set;
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::{context, Environment};
use tower_http::services::fs::ServeDir;

// Constants for configuration
const STRATA_FULLNODE: &str =
    "http://fnclient675f9eff3a682b8c0ea7423.devnet-annapurna.stratabtc.org/";
const DATABASE_URL: &str = "postgres://username@localhost:5432/checkpoints";
const FETCH_INTERVAL: u64 = 500;

#[tokio::main]
async fn main() {
    // Initialize logging with debug level enabled
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG) // Set the maximum log level to DEBUG
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Initialize database connection
    let database = Arc::new(DatabaseWrapper::new(DATABASE_URL).await);

    // Initialize fetcher
    let fetcher = Arc::new(StrataFetcher::new(STRATA_FULLNODE.to_string()));

    let (tx, rx):(Sender<CheckpointRange>, Receiver<CheckpointRange>) = mpsc::channel(100);

    // Spawn a background task for fetching checkpoint data
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    
    tokio::spawn(async move {
        info!("Starting data fetcher thread...");
        
        // Periodic fetching every 5 seconds
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(FETCH_INTERVAL));
        loop {
            let tx_clone = tx.clone();
            interval.tick().await;
            match fetch_checkpoint_from_fullnode(&fetcher_clone, &database_clone, tx_clone).await {
                Ok(_) => (),
                Err(e) => tracing::error!("Error during periodic data fetch: {}", e),
            }
        }
    });

    // Spawn a background task for fetching block data
        // Spawn a background task for fetching checkpoint data
        let fetcher_clone = fetcher.clone();
        let database_clone = database.clone();
    tokio::spawn(
        async move {
            info!("Starting block fetcher thread...");
            run_block_fetcher(fetcher_clone, database_clone, rx).await;
        }
    );

    // Initialize Jinja2 templates
    let mut env = Environment::new();
    env.add_template("base.html", include_str!("templates/base.html"))
        .unwrap();
    env.add_template("homepage.html", include_str!("templates/homepage.html"))
        .unwrap();
    env.add_template("checkpoint.html", include_str!("templates/checkpoint.html"))
        .unwrap();
    env.add_template("pagination.html", include_str!("templates/pagination.html"))
        .unwrap();
    env.add_template("navbar.html", include_str!("templates/navbar.html"))
        .unwrap();
    env.add_template("search.html", include_str!("templates/search.html"))
        .unwrap();
    env.add_template(
        "mobile-menu.html",
        include_str!("templates/mobile/menu-button.html"),
    )
    .unwrap();

    // Build the router
    let app = Router::new()
        .route("/", get(homepage))
        .route("/checkpoint", get(checkpoint_details))
        .nest_service("/static", ServeDir::new("bin/batch-explorer/src/static"))
        .with_state(database.clone())
        .layer(axum::Extension(Arc::new(env)));

    info!("Listening on 0.0.0.0:3000");

    // Start the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for the homepage
async fn homepage(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let current_page = params.p.unwrap_or(1);
    let page_size = params.ps.unwrap_or(3);

    let pagination_info = database
        .get_paginated_checkpoints(current_page, page_size, 1) // Set absolute_first_page to 1 for batch tables
        .await;

    render_template(
        &env,
        "homepage.html",
        context! {
            pagination => pagination_info, // Pass the entire struct to the template
        },
    )
}

// Handler for checkpoint details
async fn checkpoint_details(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<CheckpointQuery>,
) -> impl IntoResponse {
    let current_page = params.p.unwrap_or(0); // Default to page 0
    let page_size = 1; // Set page size

    // Get paginated checkpoints
    let mut pagination_info = database
        .get_paginated_checkpoints(current_page, page_size, 0)
        .await;
    pagination_info.total_pages -= 1; // Adjust total pages for 0-based indexing

    render_template(
        &env,
        "checkpoint.html",
        context! {
            pagination => pagination_info
        },
    )
}
// Utility function to render templates
fn render_template(
    env: &Environment<'_>,
    template_name: &str,
    context: minijinja::value::Value,
) -> Html<String> {
    let template = env.get_template(template_name).unwrap();
    let rendered = template.render(context).unwrap();
    info!("test if error from here");
    Html(rendered)
}

// Struct for pagination parameters
#[derive(Debug, Deserialize)]
struct PaginationParams {
    p: Option<u64>,
    ps: Option<u64>,
}

#[derive(Deserialize)]
struct CheckpointQuery {
    p: Option<u64>,
}

async fn fetch_checkpoint_from_fullnode(
    fetcher: &Arc<StrataFetcher>,
    database: &Arc<DatabaseWrapper>,
    tx: Sender<CheckpointRange>
) -> anyhow::Result<()> {
    info!("Fetching data from fullnode...");

    // Get the last checkpoint index from the full node
    let fullnode_last_checkpoint = fetcher.get_latest_index("strata_getLatestCheckpointIndex").await?;
    info!(
        "Fullnode last checkpoint index: {}",
        fullnode_last_checkpoint
    );

    // Get the last checkpoint index from the local database
    // If table is empty it will return -1 hence we will try to fetch from 0
    // If table is not empty it will return the last checkpoint index and fetch from the next index
    let local_last_checkpoint = database.get_latest_checkpoint_index().await.unwrap_or(-1);

    // Fetch all missing checkpoints
    for idx in (local_last_checkpoint + 1)..=fullnode_last_checkpoint as i64 {
        match fetcher.fetch_data::<RpcCheckpointInfo>("strata_getCheckpointInfo", idx).await {
            Ok(checkpoint) => {
                info!("Fetched checkpoint ID: {}", idx);
                database.insert_checkpoint(checkpoint.clone()).await;
                // Send the L2 block range to the block fetcher
                let range = CheckpointRange {
                    idx,
                    start: checkpoint.l2_range.0 as i64,
                    end: checkpoint.l2_range.0 as i64,
                };
                tx.send(range).await?;
            }
            Err(e) => {
                tracing::warn!("Failed to fetch checkpoint {}: {}", idx, e);
            }
        }
    }
    Ok(())
}


// Data structure for checkpoint range
#[derive(Debug, Deserialize, Clone)]
pub struct CheckpointRange {
    idx: i64,
    start: i64,
    end: i64,
}

/// Runs the block fetcher, which fetches blocks in the given checkpoint range
async fn run_block_fetcher(
    fetcher: Arc<StrataFetcher>,
    database: Arc<DatabaseWrapper>,
    mut rx: Receiver<CheckpointRange>,
) {
    while let Some(range) = rx.recv().await {
        info!("Received checkpoint range: {:?}", range);
        fetch_blocks_in_range(&fetcher, &database, range).await;
    }
}

/// Fetches blocks in the given range and stores them in the database
pub async fn fetch_blocks_in_range(
    fetcher: &Arc<StrataFetcher>,
    database: &Arc<DatabaseWrapper>,
    range: CheckpointRange,
) {
    let checkpoint_idx = range.idx;
    for block_height in range.start..=range.end {
        // Fetch block data from the full node
        match fetcher.fetch_data::<Vec<RpcBlockHeader>>("strata_getHeadersAtIdx", block_height).await {
            Ok(rpc_block_header) => {
                // Pass the RpcBlockHeader directly to `insert_block`
                database.insert_block(rpc_block_header[0].clone(), checkpoint_idx).await;
            }
            Err(e) => {
                tracing::warn!("Failed to fetch block {}: {}", block_height, e);
            }
        }
    }
}