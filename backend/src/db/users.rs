// src/db/users.rs
// Database operations for users

use rand::rngs::OsRng;
use rsa::{
    RsaPrivateKey,
    pkcs8::{EncodePrivateKey, EncodePublicKey},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::user::{CreateUser, CreateUserResponse, UpdateUserIntern, User};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_users(&self) -> AppResult<Vec<User>> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, pseudo, password, public_key
            FROM vaultchat.users
            ORDER BY pseudo ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> AppResult<User> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, pseudo, password, public_key
            FROM vaultchat.users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))
    }

    pub async fn create_user(
        &self,
        input: CreateUser,
        hash: &str,
    ) -> AppResult<CreateUserResponse> {
        // Generate a 2048-bit RSA key pair
        let mut rng = OsRng;
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).map_err(|e| {
            AppError::Internal(anyhow::anyhow!("Failed to generate RSA key: {}", e))
        })?;
        let pub_key = priv_key.to_public_key();

        // Encode keys to PEM strings
        let priv_key_pem = priv_key
            .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to encode private key: {}", e))
            })?
            .to_string();
        let pub_key_pem = pub_key
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .map_err(|e| {
                AppError::Internal(anyhow::anyhow!("Failed to encode public key: {}", e))
            })?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO vaultchat.users (pseudo, password, public_key)
            VALUES ($1, $2, $3)
            RETURNING id, pseudo, password, public_key
            "#,
        )
        .bind(input.username)
        .bind(hash)
        .bind(&pub_key_pem)
        .fetch_one(&self.pool)
        .await?;

        Ok(CreateUserResponse {
            user: user.to_safe(),
            private_key: priv_key_pem,
        })
    }

    pub async fn update_user(&self, input: UpdateUserIntern) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE vaultchat.users
            SET
                pseudo = COALESCE($2, pseudo),
                password = COALESCE($3, password)
            WHERE id = $1
            RETURNING id, pseudo, password, public_key
            "#,
        )
        .bind(input.id)
        .bind(input.username)
        .bind(input.password)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", input.id)))?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM vaultchat.users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_user_by_username(&self, username: String) -> AppResult<User> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, pseudo, password, public_key
            FROM vaultchat.users
            WHERE pseudo = $1
            ORDER BY pseudo ASC
            "#,
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(users)
    }
}
