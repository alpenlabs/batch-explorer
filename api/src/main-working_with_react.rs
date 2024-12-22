mod db;
mod models;
mod routes;
mod fetcher;
mod helper;
mod cache;

use axum::{routing::get, routing::post, Router};
use tower_http::cors::{CorsLayer, Any};
use fetcher::StrataFetcher;
use routes::{fetch_and_store_checkpoint, get_checkpoint, generate_sample_data, get_checkpoints_paginated};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc ;
use db::Database;
use reqwest::header::HeaderValue;
use reqwest::Method;
use reqwest::header;

// TODO: get this from config
const STRATA_FULLNODE: &str = "http://fnclient675f9eff3a682b8c0ea7423.devnet-annapurna.stratabtc.org/";
const CACHE_SIZE: usize = 1000;


#[tokio::main]
async fn main() {
    // Initialize logging
    FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .init();
    
    // Initialize RocksDB and Fetcher
    let dbs = Arc::new(Database::new("batches_db", CACHE_SIZE));
    let fetcher = Arc::new(StrataFetcher::new(STRATA_FULLNODE.to_string()));

    // Define the CORS layer
    let cors = CorsLayer::new()
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()) // Allow your frontend's origin
    .allow_methods([Method::GET, Method::POST]) // Allow specific HTTP methods
    .allow_headers([header::CONTENT_TYPE]); // Allow specific headers

    let db_router = Router::new()
        .route("/checkpoint/:q", get(get_checkpoint))
        .with_state(dbs.clone());

    let fetch_router = Router::new()
        .route("/fetch/:idx", post(fetch_and_store_checkpoint))
        .with_state((dbs.clone(), fetcher.clone()));

    let temp_generate_data = Router::new()
        .route("/generate_data/:start_idx", get(generate_sample_data))
        .with_state((dbs.clone(), fetcher.clone()));

    let checkpoints_paginated = Router::new()
        .route("/checkpoints_paginated", get(get_checkpoints_paginated))
        .layer(cors)
        .with_state(dbs.clone());

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