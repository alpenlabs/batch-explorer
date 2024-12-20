use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;
use minijinja::{Environment, context};
use std::sync::Arc;
use tower_http::services::fs::ServeDir;

#[tokio::main]
async fn main() {
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
        .layer(axum::Extension(Arc::new(env)));

    info!("Listening on 0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn homepage(axum::Extension(env): axum::Extension<Arc<Environment<'_>>>) -> impl IntoResponse {
    // Mock data
    let checkpoints = vec![
        context! {
            idx => 1,
            l1_range => [100, 115],  // Ensure this is an array
            l2_range => [1, 1],
            l2_blockid => "295295a50a0b1234567890abcdef1234567890abcdef",
        },
        context! {
            idx => 2,
            l1_range => [116, 130],
            l2_range => [2, 2],
            l2_blockid => "8aa000a814a71234567890abcdef1234567890abcd",
        },
    ];

    let template = env.get_template("homepage.html").unwrap();
    let rendered = template
        .render(context! {
            checkpoints => checkpoints,
            current_page => 1,
            total_pages => 1,
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