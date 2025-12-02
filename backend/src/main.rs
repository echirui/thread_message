mod db;
mod error;
mod handlers;
mod models;
mod ws;

use axum::{
    routing::{get, post},
    Router,
    response::Json,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tokio::sync::broadcast;
use crate::models::Message;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub tx: broadcast::Sender<Message>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenvy::dotenv().ok();

    // Init tracing
    tracing_subscriber::fmt::init();

    // Init DB
    let pool = db::init_pool().await?;

    // Init broadcast channel
    let (tx, _rx) = broadcast::channel(100);

    // Create State
    let state = AppState { pool, tx };

    // Build app
    let app = Router::new()
        .route("/", get(health_check))
        .route("/messages", get(handlers::chat::get_messages).post(handlers::chat::create_message))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

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