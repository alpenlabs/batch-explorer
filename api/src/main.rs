mod db;
mod models;
mod routes;
mod fetcher;
mod helper;
mod cache;

use fetcher::StrataFetcher;
use routes::{fetch_and_store_checkpoint, get_checkpoint, generate_sample_data, get_checkpoints_paginated, PaginationParams};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc ;
use db::Database;

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
        .route("/checkpoint", get(checkpoint_details))
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
    // Set default pagination values
    let page = params.p.unwrap_or(1);
    let page_size = params.ps.unwrap_or(10);

    // Fetch paginated checkpoints
    let paginated_checkpoints = get_checkpoints_paginated(
        state,
        PaginationParams {
            p: Some(page),
            ps: Some(page_size),
        },
    )
    .await;

    // Render the template
    render_template(
        &env,
        "homepage.html",
        context! {
            checkpoints => paginated_checkpoints.checkpoints,
            current_page => paginated_checkpoints.current_page,
            total_pages => paginated_checkpoints.total_pages,
        },
    )
}

async fn checkpoint_details(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    state: axum::extract::State<Arc<Database>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let id = params.p.unwrap_or(1);
    
    if let Some(checkpoint) = state.get_checkpoint_by_idx(id) {
        // Fetch total checkpoints for pagination
        let total_checkpoints = state.get_total_checkpoint_count();

        // Calculate the current page and total pages for navigation
        let current_page = id;
        let total_pages = total_checkpoints - 1 ;

        let current_path = "/checkpoint";

        return render_template(
            &env,
            "checkpoint.html",
            context! {
                checkpoint,
                current_page,
                total_pages,
                current_path,
            },
        );
    }
    
    render_template(&env, "404.html", context! { message => "Checkpoint not found" })
}

fn render_template(
    env: &Environment<'_>,
    template_name: &str,
    context: minijinja::value::Value,
) -> Html<String> {
    let template = env.get_template(template_name).unwrap();
    let rendered = template.render(context).unwrap();
    Html(rendered)
}