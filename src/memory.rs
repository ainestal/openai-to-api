use anyhow::Result;
use sqlx::{types::time::PrimitiveDateTime, PgPool};

#[derive(Debug)]
pub struct Memory {
    pub id: i32,
    pub created_at: PrimitiveDateTime,
    pub content: String,
    pub session_id: String,
}

impl Memory {
    pub async fn new(content: String, session_id: String, pool: &PgPool) -> Result<Memory> {
        let row = sqlx::query!(
            r#"
        INSERT INTO memory (content, role, session_id)
        VALUES ($1, 'assistant', $2)
        RETURNING id, created_at, content, session_id
        "#,
            content,
            session_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Memory {
            id: row.id,
            created_at: row.created_at,
            content: row.content,
            session_id: row.session_id,
        })
    }

    pub async fn read_by_id(id: String, pool: &PgPool) -> Result<Memory> {
        // Transform id string to id i32
        let id = id.parse::<i32>()?;
        let row = sqlx::query!(
            r#"
        SELECT id, created_at, content, session_id
        FROM memory
        WHERE id = $1
        AND archived = false
        "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Memory {
            id: row.id,
            created_at: row.created_at,
            content: row.content,
            session_id: row.session_id,
        })
    }
}
