use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;
use crate::error::Result;

pub async fn init_pool() -> Result<SqlitePool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
