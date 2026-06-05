// src/models/chat.rs
// This module defines the Message and User struct with related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
pub struct UpdateUserIntern {
    pub id: Uuid,
    pub pseudo: Option<String>,
    pub password: Option<String>,
}

pub struct UpdateUserExtern  {
    pub pseudo: Option<String>,
    pub password: Option<String>,
}

