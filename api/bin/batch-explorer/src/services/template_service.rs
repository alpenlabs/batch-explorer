use axum::{extract::{State, Query}, response::{Html,IntoResponse}};
use database::db::DatabaseWrapper;
use minijinja::{context, Environment, Value};
use std::sync::Arc;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

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
pub async fn checkpoint_details(
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
    Html(rendered)
}

// Struct for pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    p: Option<u64>,
    ps: Option<u64>,
}

#[derive(Deserialize)]
pub struct CheckpointQuery {
    p: Option<u64>,
}
