// src/models/chat.rs
// This module defines the Message and User struct with related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: Option<String>,
    pub sent_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessage {
    pub sender_id: Uuid,
    pub receiver_id: Option<Uuid>,
    pub receiver_pseudo: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageIntern {
    pub id: Uuid,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessageExtern {
    pub content: Option<String>,
}
