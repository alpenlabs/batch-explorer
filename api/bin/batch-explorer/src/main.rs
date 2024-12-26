mod fetcher;
mod helper;

use fetcher::StrataFetcher;
use service::db::DatabaseWrapper;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::{Environment, context};
use tower_http::services::fs::ServeDir;

// Constants for configuration
const STRATA_FULLNODE: &str = "http://fnclient675f9eff3a682b8c0ea7423.devnet-annapurna.stratabtc.org/";
const CACHE_SIZE: usize = 1000;
const DATABASE_URL: &str = "postgres://username@localhost:5432/checkpoints";

#[tokio::main]
async fn main() {
    // Initialize logging
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .init();

    // Initialize database connection
    let database = Arc::new(DatabaseWrapper::new(DATABASE_URL).await);

    // Initialize Jinja2 templates
    let mut env = Environment::new();
    env.add_template("base.html", include_str!("templates/base.html")).unwrap();
    env.add_template("homepage.html", include_str!("templates/homepage.html")).unwrap();
    env.add_template("checkpoint.html", include_str!("templates/checkpoint.html")).unwrap();
    env.add_template("pagination.html", include_str!("templates/pagination.html")).unwrap();
    env.add_template("navbar.html", include_str!("templates/navbar.html")).unwrap();
    env.add_template("search.html", include_str!("templates/search.html")).unwrap();
    env.add_template("mobile-menu.html", include_str!("templates/mobile/menu-button.html")).unwrap();

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
    let page_size = 1;                      // Set page size

    // Get paginated checkpoints
    let pagination_info = database
        .get_paginated_checkpoints(current_page, page_size, 0)
        .await;

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

