pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod ws;

use axum::{
    http::{Method, HeaderValue},
    routing::{get, post},
    Router,
    response::Json,
};
use serde_json::{json, Value};
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use tower_cookies::CookieManagerLayer;
use tokio::sync::broadcast;
use crate::models::Message;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub tx: broadcast::Sender<Message>,
}

impl axum::extract::FromRef<AppState> for sqlx::SqlitePool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

pub async fn create_app(pool: sqlx::SqlitePool) -> Router {
    // Init broadcast channel
    let (tx, _rx) = broadcast::channel(100);

    // Create State
    let state = AppState { pool, tx };

    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/", get(health_check))
        .route("/messages", get(handlers::chat::get_messages))
        .route("/messages", post(handlers::chat::create_message))
        .route("/messages/:id/replies", get(handlers::thread::get_thread_replies))
        .route("/ws", get(ws::ws_handler))
        .route("/auth/login", get(handlers::auth::login))
        .route("/auth/callback", get(handlers::auth::callback))
        .route("/auth/me", get(handlers::auth::get_me))
        .route("/auth/logout", post(handlers::auth::logout))
        .with_state(state)
        .layer(CookieManagerLayer::new())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}