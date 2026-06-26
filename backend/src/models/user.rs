// src/models/chat.rs
// This module defines the Message and User struct with related types

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow)]
pub struct User {
    pub id: Uuid,
    #[sqlx(rename = "pseudo")]
    pub username: String,
    pub password: String,
    pub public_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user: User,
    pub private_key: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserIntern {
    pub id: Uuid,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserExtern  {
    pub username: Option<String>,
    pub password: Option<String>,
}
