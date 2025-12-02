use axum::{
    extract::State,
    Json,
};
use crate::{
    error::Result,
    models::{CreateMessage, Message},
    AppState,
};

pub async fn get_messages(
    State(state): State<AppState>,
) -> Result<Json<Vec<Message>>> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE parent_id IS NULL ORDER BY created_at ASC"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(messages))
}

pub async fn create_message(
    State(state): State<AppState>,
    Json(payload): Json<CreateMessage>,
) -> Result<Json<Message>> {
    let message = sqlx::query_as::<_, Message>(
        "INSERT INTO messages (content, sender_id, parent_id) VALUES (?, ?, ?) RETURNING *"
    )
    .bind(&payload.content)
    .bind(&payload.sender_id)
    .bind(payload.parent_id) // & を削除
    .fetch_one(&state.pool)
    .await?;

    // Broadcast message
    let _ = state.tx.send(message.clone());

    Ok(Json(message))
}
