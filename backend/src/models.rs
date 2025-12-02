use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Message {
    pub id: i64,
    pub content: String,
    pub sender_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMessage {
    pub content: String,
    pub sender_id: String,
    pub parent_id: Option<i64>,
}
