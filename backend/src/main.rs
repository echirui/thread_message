mod db;
mod error;

use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Init tracing
    tracing_subscriber::fmt::init();

    // Init DB
    let pool = db::init_pool().await?;
    
    // Create State
    let state = AppState { pool };

    // Build app
    let app = Router::new()
        .route("/", get(health_check))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
