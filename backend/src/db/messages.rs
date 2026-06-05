// src/db/messages.rs
// Database operations for chat messages

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::vault_chat::{CreateMessage, Message, UpdateMessageIntern};

#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_chat_messages(
        &self,
        sender_id: Uuid,
        receiver_id: Uuid,
    ) -> AppResult<Vec<Message>> {
        let chat_messages = sqlx::query_as::<_, Message>(
            r#"
            SELECT id, sender_id, receiver_id, content, sent_at
            FROM messages
            WHERE (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
            ORDER BY sent_at ASC
            "#,
        )
        .bind(sender_id)
        .bind(receiver_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(chat_messages)
    }

    pub async fn get_message_by_id(&self, id: Uuid) -> AppResult<Message> {
        sqlx::query_as::<_, Message>(
            r#"
            SELECT id, sender_id, receiver_id, content, sent_at
            FROM messages
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Message with id {} not found", id)))
    }

    pub async fn create_message(&self, input: CreateMessage) -> AppResult<Message> {
        let now = Utc::now();

        let message = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (sender_id, receiver_id, content, sent_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, sender_id, receiver_id, content, sent_at
            "#,
        )
        .bind(&input.sender_id)
        .bind(&input.receiver_id)
        .bind(&input.content)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    pub async fn update_message(&self, input: UpdateMessageIntern) -> AppResult<Message> {
        let message = sqlx::query_as::<_, Message>(
            r#"
            UPDATE messages
            SET
                content = COALESCE($2, content)
            WHERE id = $1
            RETURNING id, sender_id, receiver_id, content, sent_at
            "#,
        )
        .bind(input.id)
        .bind(input.content)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Message with id {} not found", input.id)))?;

        Ok(message)
    }

    pub async fn delete_message(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM messages WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}