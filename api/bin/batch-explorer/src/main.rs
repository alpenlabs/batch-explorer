mod fetcher;
mod helper;

use fetcher::StrataFetcher;
use serde::Deserialize;
use service::db::DatabaseWrapper;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
const FETCH_INTERVAL: u64 = 5;

#[tokio::main]
async fn main() {
    // Initialize logging
    FmtSubscriber::builder().with_max_level(Level::INFO).init();

    // Initialize database connection
    let database = Arc::new(DatabaseWrapper::new(DATABASE_URL).await);

    // Initialize fetcher
    let fetcher = Arc::new(StrataFetcher::new(STRATA_FULLNODE.to_string()));

    // Spawn a background task for fetching checkpoint data
    let fetcher_clone = fetcher.clone();
    let database_clone = database.clone();
    tokio::spawn(async move {
        info!("Starting data fetcher thread...");

        // Periodic fetching every 5 seconds
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(FETCH_INTERVAL));
        loop {
            interval.tick().await;
            match fetch_checkpoint_from_fullnode(&fetcher_clone, &database_clone).await {
                Ok(_) => (),
                Err(e) => tracing::error!("Error during periodic data fetch: {}", e),
            }
        }
    });

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
) -> anyhow::Result<()> {
    info!("Fetching data from fullnode...");

    // Get the last checkpoint index from the full node
    let fullnode_last_checkpoint = fetcher.get_last_checkpoint_index().await?;
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
        match fetcher.fetch_checkpoint(idx).await {
            Ok(checkpoint) => {
                info!("Fetched checkpoint ID: {}", idx);
                database.insert_checkpoint(checkpoint).await;
            }
            Err(e) => {
                tracing::warn!("Failed to fetch checkpoint {}: {}", idx, e);
            }
        }
    }
    Ok(())
}
