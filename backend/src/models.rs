use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Message {
    pub id: i64,
    pub content: String,
    pub sender_id: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessage {
    pub content: String,
    pub sender_id: String,
}
