// src/models/chat.rs
// This module defines the Message and User struct with related types

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::models::vault_chat::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromRow)]
pub struct User {
    pub id: Uuid,
    #[sqlx(rename = "pseudo")]
    pub username: String,
    pub password: String,
    pub public_key: String,
}

impl User {
    pub fn to_safe(&self) -> SafeUser {
        SafeUser {
            id: self.id,
            username: self.username.clone(),
            public_key: self.public_key.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SafeUser {
    pub id: Uuid,
    #[sqlx(rename = "pseudo")]
    pub username: String,
    pub public_key: String,
}

impl SafeUser {
    pub fn to_contact(&self, last_message: Option<Message>) -> Contact {
        Contact {
            id: self.id,
            username: self.username.clone(),
            public_key: self.public_key.clone(),
            last_message,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user: SafeUser,
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
pub struct UpdateUserExtern {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Contact {
    pub id: Uuid,
    pub username: String,
    pub public_key: String,
    pub last_message: Option<Message>,
}
