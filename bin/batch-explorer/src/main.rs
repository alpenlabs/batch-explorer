mod services;
mod utils;

use axum::{routing::get, Router};
use database::connection::DatabaseWrapper;
use fullnode_client::fetcher::StrataFetcher;
use tower_http::services::ServeDir;
use services::{block_service::run_block_fetcher, checkpoint_service::start_checkpoint_fetcher, template_service::initialize_templates};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use utils::config::Config;
use dotenvy::dotenv;
use clap::Parser;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::parse();

    // Initialize logging
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()) // <-- enables RUST_LOG
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set logging subscriber");

    // Initialize database and fetcher
    let database = Arc::new(DatabaseWrapper::new(&config.database_url).await);
    let fetcher = Arc::new(StrataFetcher::new(config.strata_fullnode));

    // Channels for communication between checkpoint fetcher and block fetcher
    let (tx, rx) = mpsc::channel(100);

    // Start block fetcher task
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    tokio::spawn(async move {
        run_block_fetcher(fetcher_clone, database_clone, rx).await;
    });
    
    // Start checkpoint fetcher task
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    tokio::spawn(async move {
        start_checkpoint_fetcher(fetcher_clone, database_clone, tx, config.fetch_interval).await;
    });


    // Initialize Jinja2 templates
    let env = initialize_templates();

    // Setup Axum router
    let app = Router::new()
        .route("/", get(services::template_service::homepage))
        .route("/checkpoint", get(services::template_service::checkpoint_details))
        .route("/search", get(services::template_service::search_handler))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(database.clone())
        .layer(axum::Extension(Arc::new(env)));

    // Start the server
    let addr = "0.0.0.0:3000".parse().unwrap();
    info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}