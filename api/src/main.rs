mod db;
mod models;
mod routes;
mod fetcher;
mod helper;

use axum::{routing::get, routing::post, Router};
// use db::Database;
// use fetcher::StrataFetcher;
use routes::{fetch_and_store_checkpoint, get_checkpoint, generate_sample_data, get_checkpoints_paginated};
use std::sync::Arc;
// use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

const STRATA_FULLNODE: &str = "http://fnclient675f9eff3a682b8c0ea7423.devnet-annapurna.stratabtc.org/";

#[tokio::main]
async fn main() {
    // Initialize logging
    FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .init();
    
    // Initialize RocksDB and Fetcher
    let db = Arc::new(db::Database::new("batches_db"));
    let fetcher = Arc::new(fetcher::StrataFetcher::new(STRATA_FULLNODE.to_string()));

    // Create sub-routers with their respective states
    let db_router = Router::new()
        .route("/checkpoint/:q", get(get_checkpoint))
        .with_state(db.clone());

    let fetch_router = Router::new()
        .route("/fetch/:idx", post(fetch_and_store_checkpoint))
        .with_state((db.clone(), fetcher.clone()));

    let temp_generate_data = Router::new()
        .route("/generate_data/:start_idx", get(generate_sample_data))
        .with_state((db.clone(), fetcher.clone()));

    let checkpoints_paginated = Router::new()
        .route("/checkpoints_paginated", get(get_checkpoints_paginated))
        .with_state(db.clone());

    // Combine sub-routers into the main app
    let app = db_router
        .merge(fetch_router)
        .merge(temp_generate_data)
        .merge(checkpoints_paginated);

    // Start server
    info!("Server started at: http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}