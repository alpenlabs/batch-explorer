mod db;
mod models;
mod routes;
mod fetcher;
mod helper;
mod cache;

use tower_http::cors::{CorsLayer, Any};
use fetcher::StrataFetcher;
use routes::{fetch_and_store_checkpoint, get_checkpoint, generate_sample_data, get_checkpoints_paginated, PaginationParams};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc ;
use db::Database;
use reqwest::header::HeaderValue;
use reqwest::Method;
use reqwest::header;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::{Environment, context};
use tower_http::services::fs::ServeDir;

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

    let mut env = Environment::new();
    env.add_template("base.html", include_str!("templates/base.html")).unwrap();
    env.add_template("homepage.html", include_str!("templates/homepage.html")).unwrap();
    env.add_template("checkpoint.html", include_str!("templates/checkpoint.html")).unwrap();
    env.add_template("pagination.html", include_str!("templates/pagination.html")).unwrap();
    env.add_template("navbar.html", include_str!("templates/navbar.html")).unwrap();
    env.add_template("search.html", include_str!("templates/search.html")).unwrap();
    env.add_template("mobile-menu.html", include_str!("templates/mobile/menu-button.html")).unwrap();

    let app = Router::new()
        .route("/", get(homepage))
        .route("/checkpoint/:id", get(checkpoint_details))
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(axum::Extension(Arc::new(env)))
        .with_state(dbs.clone());

    info!("Listening on 0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn homepage(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    state: axum::extract::State<Arc<Database>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    // Set default pagination values if not provided
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(3);

    // Fetch paginated checkpoints
    let pp = PaginationParams {
        page: Some(page),
        page_size: Some(page_size),
    };
    
    let paginated_checkpoints = get_checkpoints_paginated(state, pp).await;


    let template = env.get_template("homepage.html").unwrap();
    let rendered = template
        .render(context! {
            checkpoints => paginated_checkpoints.checkpoints,
            current_page => paginated_checkpoints.current_page,
            total_pages => paginated_checkpoints.total_pages,
        })
        .unwrap();

    Html(rendered)
}

async fn checkpoint_details(
    Path(id): Path<String>,
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
) -> impl IntoResponse {
    let checkpoint = context! {
        id => id,
        batch_txid => "abc123...",
        epoch_index => 12,
        status => "confirmed",
        signet_start_block => 1000,
        signet_end_block => 1100,
        strata_start_block => 1200,
        strata_end_block => 1300,
        transactions => [
            context! { txid => "tx123", amount => 1.5 },
            context! { txid => "tx456", amount => 2.0 }
        ]
    };

    let template = env.get_template("checkpoint.html").unwrap();
    let rendered = template.render(context! { checkpoint }).unwrap();

    Html(rendered)
}