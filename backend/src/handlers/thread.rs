use axum::{
    extract::{Path, State},
    Json,
};
use crate::{
    error::Result, // AppError を削除
    models::Message,
    AppState,
};

pub async fn get_thread_replies(
    Path(parent_id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Message>>> {
    let messages = sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE parent_id = ? ORDER BY created_at ASC"
    )
    .bind(parent_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(messages))
}