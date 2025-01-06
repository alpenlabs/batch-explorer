use axum::{extract::{State, Query}, response::{Html,IntoResponse, Redirect}};
use database::connection::DatabaseWrapper;
use minijinja::{context, Environment, Value};
use std::sync::Arc;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use url::Url;
use model::pgu64::PgU64;
use database::services::{checkpoint_service::CheckpointService, block_service::BlockService};
macro_rules! template_path {
    ($file:expr) => {
        concat!("../../../../templates/", $file)
    };
}
pub fn initialize_templates() -> Environment<'static> {
    let mut env = Environment::new();

    // Load environment variables
    let mut global_context = HashMap::new();
    global_context.insert("mempool_url", env::var("MEMPOOL_URL").unwrap_or_default());
    global_context.insert("blockscout_url", env::var("BLOCKSCOUT_URL").unwrap_or_default());
    global_context.insert("strata_docs", env::var("STRATA_DOCS").unwrap_or_default());
    global_context.insert("strata_blog", env::var("STRATA_BLOG").unwrap_or_default());
    global_context.insert("strata_url", env::var("STRATA_URL").unwrap_or_default());

    env.add_global("env", Value::from(global_context)); // Add global context for all templates
    
    env.add_template("base.html", include_str!(template_path!("base.html"))).unwrap();
    env.add_template("homepage.html", include_str!(template_path!("homepage.html"))).unwrap();
    env.add_template("search.html", include_str!(template_path!("search.html"))).unwrap();
    env.add_template("checkpoint.html", include_str!(template_path!("checkpoint.html"))).unwrap();
    env.add_template("pagination.html", include_str!(template_path!("pagination.html"))).unwrap();
    env.add_template("navbar.html", include_str!(template_path!("navbar.html"))).unwrap();
    env.add_template("mobile-menu.html", include_str!(template_path!("mobile-menu.html"))).unwrap();
    env
}

// Handler for the homepage
pub async fn homepage(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let current_page = params.p.unwrap_or(1);
    let page_size = params.ps.unwrap_or(10);
    let error_msg = params.error_msg.clone();
    tracing::info!("error_msg: {:?}", error_msg);

    let checkpoint_db = CheckpointService::new(&database.db);
    let pagination_info = checkpoint_db
        .get_paginated_checkpoints(current_page, page_size, 1) // Set absolute_first_page to 1 for batch tables
        .await;

    render_template(
        &env,
        "homepage.html",
        context! {
            pagination => pagination_info, // Pass the entire struct to the template
            error_msg => error_msg,
        },
    )
}

// Handler for checkpoint details
pub async fn checkpoint_details(
    axum::Extension(env): axum::Extension<Arc<Environment<'_>>>,
    State(database): State<Arc<DatabaseWrapper>>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let current_page = params.p.unwrap_or(0); // Default to page 0
    let page_size = 1; // Set page size

    let checkpoint_db = CheckpointService::new(&database.db);
    // Get paginated checkpoints
    let mut pagination_info = checkpoint_db
        .get_paginated_checkpoints(current_page, page_size, 0)
        .await;
    pagination_info.total_pages -= 1; // Adjust total pages for 0-based indexing

    render_template(
        &env,
        "checkpoint.html",
        context! {
            pagination => pagination_info,
            error_msg => params.error_msg,
        },
    )
}

use axum::headers::HeaderMap;


#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

pub async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(database): State<Arc<DatabaseWrapper>>,
    headers: HeaderMap, // Extract headers from the request
) -> impl IntoResponse {
    let mut query = params.query.trim();
    let checkpoint_db = CheckpointService::new(&database.db);
    
    // Check if it's a valid block number
    if let Ok(block_number) = query.parse::<u64>() {
        tracing::info!("Searching for block number: {}", block_number);
        let block_number = PgU64(block_number).to_i64();
        if let Ok(Some(checkpoint_idx)) = checkpoint_db.get_checkpoint_idx_by_block_height(block_number).await {
            let checkpoint_idx = PgU64::from_i64(checkpoint_idx).0;
            // Redirect to the batch page if found
            return Redirect::to(format!("/checkpoint?p={}", checkpoint_idx).as_str());
        }
    }

    // Check if it's a valid block hash
    tracing::info!("Searching for block hash: {}", query);

    // Remove the "0x" prefix if present
    if query.starts_with("0x") {
        query = query.trim_start_matches("0x");
    }
    if let Ok(Some(checkpoint_idx)) = checkpoint_db.get_checkpoint_idx_by_block_hash(query).await {
        // Redirect to the batch page if found
            let checkpoint_idx = PgU64::from_i64(checkpoint_idx).0;
            return Redirect::to(format!("/checkpoint?p={}", checkpoint_idx).as_str());
    }

    // Redirect back to the Referer with the error message
    if let Some(referer) = headers.get("Referer").and_then(|v| v.to_str().ok()) {
        tracing::info!("Referer: {}", referer);
        if let Ok(mut url) = Url::parse(referer) {
            // Remove any existing `error_msg` parameter
            url.query_pairs_mut()
                .clear()
                .append_pair("error_msg", "Invalid search entry");

            let redirect_url = url.as_str().to_string();
            tracing::info!("Redirecting to: {}", redirect_url);
            return Redirect::to(redirect_url.as_str());
        }
    }

    // If Referer is unavailable, fallback to the homepage
    let redirect_url = "/?error_msg=Invalid%20search%20entry".to_string();
    tracing::info!("Redirecting to: {}", redirect_url);
    Redirect::to(redirect_url.as_str())
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
pub struct QueryParams {
    p: Option<u64>,
    ps: Option<u64>,
    error_msg: Option<String>,
}