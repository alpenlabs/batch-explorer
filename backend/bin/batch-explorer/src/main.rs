mod services;
mod utils;
// mod cors;

use axum::{routing::get, Router, http::Method};
use database::connection::DatabaseWrapper;
use fullnode_client::fetcher::StrataFetcher;
use tower_http::services::ServeDir;
use services::{block_service::run_block_fetcher, checkpoint_service::{start_checkpoint_status_updater_task, start_checkpoint_fetcher}, template_service::initialize_templates};
use std::sync::Arc;
use tokio::sync::mpsc;
use tower_http::cors::{CorsLayer, Any};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utils::config::Config;
use dotenvy::dotenv;
use clap::Parser;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::parse();

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logging subscriber");

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

    // Start checkpoint status updater task
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    tokio::spawn(async move {
        // TODO: Make the interval configurable, for now set to 5 minutes
        start_checkpoint_status_updater_task(fetcher_clone, database_clone, 300).await;
    }); 

    // Initialize Jinja2 templates
    let env = initialize_templates();

    // Define CORS rules
    let cors = CorsLayer::new()
        .allow_origin(Any) // FIXME: should this be restrictive?
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .expose_headers(Any);

    // api routes
    let api_routes = Router::new()
        .route("/checkpoints", get(services::api_service::checkpoints))
        .route("/checkpoint", get(services::api_service::checkpoint))
        .route("/search", get(services::api_service::search));

    // Setup Axum router
    let app: Router = Router::new()
        .route("/", get(services::template_service::homepage))
        .route("/checkpoint", get(services::template_service::checkpoint_details))
        .route("/search", get(services::template_service::search_handler))
        .nest_service("/static", ServeDir::new("static"))
        .nest("/api", api_routes)
        .with_state(database.clone())
        .layer(axum::Extension(Arc::new(env)))
        .layer(cors);

    // Start the server
    let addr = "0.0.0.0:3000".parse().unwrap();
    info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}