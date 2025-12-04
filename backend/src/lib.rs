pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod ws;

use axum::{
    routing::{get, post}, // post も必要なので残す
    Router,
    response::Json,
};
use serde_json::{json, Value};
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tokio::sync::broadcast;
use crate::models::Message;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub tx: broadcast::Sender<Message>,
}

pub async fn create_app(pool: sqlx::SqlitePool) -> Router {
    // Init broadcast channel
    let (tx, _rx) = broadcast::channel(100);

    // Create State
    let state = AppState { pool, tx };

    Router::new()
        .route("/", get(health_check))
        .route("/messages", get(handlers::chat::get_messages)) // GET
        .route("/messages", post(handlers::chat::create_message)) // POST
        .route("/messages/:id/replies", get(handlers::thread::get_thread_replies))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}