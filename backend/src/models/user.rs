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
    pub wrapped_private_key: String,
    pub crypto_salt: String,
    pub aes_iv: String,
}

impl User {
    pub fn to_safe(&self) -> SafeUser {
        SafeUser {
            id: self.id,
            username: self.username.clone(),
            public_key: self.public_key.clone(),
        }
    }

    pub fn to_login(&self) -> LoginUserOutput {
        LoginUserOutput {
            id: self.id,
            username: self.username.clone(),
            public_key: self.public_key.clone(),
            wrapped_private_key: self.wrapped_private_key.clone(),
            crypto_salt: self.crypto_salt.clone(),
            aes_iv: self.aes_iv.clone(),
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginUserOutput {
    pub id: Uuid,
    #[sqlx(rename = "pseudo")]
    pub username: String,
    pub public_key: String,
    pub wrapped_private_key: String,
    pub crypto_salt: String,
    pub aes_iv: String,
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
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub public_key: String,
    pub wrapped_private_key: String,
    pub crypto_salt: String,
    pub aes_iv: String,
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

#[derive(Debug, Deserialize)]
pub struct LoginUserInput {
    pub username: String,
    pub password: String,
}
