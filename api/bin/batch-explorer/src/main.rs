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
        .route("/checkpoint/:id", get(checkpoint_details))
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
    let page = params.p.unwrap_or(1);
    let page_size = params.ps.unwrap_or(3);

    // Fetch paginated checkpoints
    let paginated_checkpoints = database
        .get_paginated_checkpoints((page - 1) * page_size, page_size)
        .await;

    // Render the template
    render_template(
        &env,
        "homepage.html",
        context! {
            checkpoints => paginated_checkpoints,
            current_page => page,
            total_pages => (database.get_total_checkpoint_count().await as f64 / page_size as f64).ceil() as u64,
        },
    )
}

// Handler for checkpoint details
async fn checkpoint_details(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    State(database): State<Arc<DatabaseWrapper>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    if let Some(checkpoint) = database.get_checkpoint_by_idx(id).await {
        // Render the template with checkpoint details
        render_template(
            &env,
            "checkpoint.html",
            context! {
                checkpoint,
                current_page => id,
                total_pages => database.get_total_checkpoint_count().await,
                current_path => format!("/checkpoint/{}", id),
            },
        )
    } else {
        // Render a 404 page if checkpoint not found
        render_template(&env, "404.html", context! { message => "Checkpoint not found" })
    }
}

// Utility function to render templates
fn render_template(
    env: &Environment<'_>,
    template_name: &str,
    context: minijinja::value::Value,
) -> Html<String> {
    let template = env.get_template(template_name).unwrap();
    let rendered = template.render(context).unwrap();
    Html(rendered)
}

// Struct for pagination parameters
#[derive(Debug, Deserialize)]
struct PaginationParams {
    p: Option<u64>,
    ps: Option<u64>,
}