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
    pub receiver_id: Uuid,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMessage {
    pub id: Uuid,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow)]
pub struct User {
    pub id: Uuid,
    pub pseudo: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub pseudo: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub id: Uuid,
    pub pseudo: Option<String>,
    pub password: Option<String>,
}
