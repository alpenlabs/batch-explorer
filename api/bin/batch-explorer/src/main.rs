mod services;

use axum::{routing::get, Router};
use config::Config as ConfigLoader;
use database::db::DatabaseWrapper;
use fullnode_client::fetcher::StrataFetcher;
use tower_http::services::ServeDir;
use services::{block_service::run_block_fetcher, checkpoint_service::start_checkpoint_fetcher, template_service::initialize_templates};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::env;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub strata_fullnode: String,
    pub database_url: String,
    pub fetch_interval: u64,
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Detect current environment
    let current_env = env::var("APP_ENV").unwrap_or_else(|_| "default".to_string());
    println!("Current environment: {}", current_env);

    // Load configuration
    let raw_config = ConfigLoader::builder()
        .add_source(config::File::with_name("config").required(false))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .expect("Failed to load configuration");

    println!("Raw configuration: {:?}", raw_config);

    // Extract the configuration for the current environment
    let config: Config = raw_config.get("default")
        .expect("Failed to extract the 'default' section from the configuration");

    println!("Loaded configuration: {:?}", config);

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set logging subscriber");

    // Initialize database and fetcher
    let database = Arc::new(DatabaseWrapper::new(&config.database_url).await);
    let fetcher = Arc::new(StrataFetcher::new(config.strata_fullnode));

    // Channels for communication between checkpoint fetcher and block fetcher
    let (tx, rx) = mpsc::channel(100);

    // Start checkpoint fetcher task
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    tokio::spawn(async move {
        start_checkpoint_fetcher(fetcher_clone, database_clone, tx, config.fetch_interval).await;
    });

    // Start block fetcher task
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    tokio::spawn(async move {
        run_block_fetcher(fetcher_clone, database_clone, rx).await;
    });

    // Initialize Jinja2 templates
    let env = initialize_templates();

    // Setup Axum router
    let app = Router::new()
        .route("/", get(services::template_service::homepage))
        .route("/checkpoint", get(services::template_service::checkpoint_details))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(database.clone())
        .layer(axum::Extension(Arc::new(env)));

    // Start the server
    let addr = "0.0.0.0:3000".parse().unwrap();
    info!("Listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}