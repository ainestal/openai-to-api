use anyhow::Result;
use sqlx::{self, PgPool};

pub async fn store_message(
    role: String,
    content: String,
    session_id: String,
    pool: &PgPool,
) -> Result<i32> {
    let row = sqlx::query!(
        r#"
    INSERT INTO message (role, content, session_id)
    VALUES ($1, $2, $3)
    RETURNING id
    "#,
        role,
        content,
        session_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}
