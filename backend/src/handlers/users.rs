// src/handlers/users.rs
// HTTP request handlers for users operations

use std::str::FromStr;

use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::user::SafeUser,
};
use crate::{
    helpers::password::hash_password,
    models::user::{UpdateUserExtern, UpdateUserIntern},
};
use crate::{helpers::password::verify_password, models::user::CreateUser};
use crate::{
    security::auth::{Claims, JWT_SECRET, create_jwt},
    state::VaultChatState,
};

// GET /users - List users
pub async fn list_users(State(state): State<VaultChatState>) -> AppResult<Json<Vec<SafeUser>>> {
    let users = state.user_repo.get_all_users().await?;
    let mut safe_users = vec![];
    for user in users {
        safe_users.push(user.to_safe());
    }
    Ok(Json(safe_users))
}

// GET /user/:id - Get an user by id
pub async fn get_user_by_id(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<SafeUser>> {
    let user = state.user_repo.get_user_by_id(id).await?;
    Ok(Json(user.to_safe()))
}

// POST /users - Create a new user
pub async fn create_user(
    State(state): State<VaultChatState>,
    Json(input): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<crate::models::user::CreateUserResponse>)> {
    if input.username.trim().is_empty() {
        return Err(AppError::Validation("username cannot be empty".to_string()));
    }

    if input.password.trim().is_empty() {
        return Err(AppError::Validation("Password cannot be empty".to_string()));
    }

    let hash = match hash_password(input.password.as_str()) {
        Ok(hash) => hash,
        Err(err) => {
            return Err(AppError::Validation(
                format!("Error while hashing password: {:?}", err).to_string(),
            ));
        }
    };

    let response = state.user_repo.create_user(input, hash.as_str()).await?;

    Ok((StatusCode::CREATED, Json(response)))
}

// PATCH /users/:id - Update an user
pub async fn update_user(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateUserExtern>,
) -> AppResult<Json<SafeUser>> {
    let new_user = UpdateUserIntern {
        id,
        username: input.username,
        password: input.password,
    };

    let user = state.user_repo.update_user(new_user).await?;
    Ok(Json(user.to_safe()))
}

// DELETE /users/:id - Delete an user
pub async fn delete_user(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = state.user_repo.delete_user(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("User with id {} not found", id)))
    }
}

pub async fn login_user(
    State(state): State<VaultChatState>,
    jar: CookieJar,
    Json(input): Json<CreateUser>,
) -> AppResult<(CookieJar, Json<SafeUser>)> {
    let user = state.user_repo.get_user_by_username(input.username).await?;

    if !verify_password(&user.password, &input.password) {
        return Err(AppError::NotFound(
            "Pseudo or password incorrect".to_string(),
        ));
    }

    let token = create_jwt(&user.id.to_string())
        .map_err(|e| AppError::Internal(anyhow!("JWT Error: {}", e)))?;

    let cookie = Cookie::build(("vaultchat_session", token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();

    Ok((jar.add(cookie), Json(user.to_safe())))
}

pub async fn get_current_user(
    State(state): State<VaultChatState>,
    jar: CookieJar,
) -> AppResult<Json<SafeUser>> {
    let cookie = jar
        .get("vaultchat_session")
        .ok_or_else(|| AppError::NotFound("No session cookie found".to_string()))?;

    let token_data = decode::<Claims>(
        cookie.value(),
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|e| AppError::Internal(anyhow!("Invalid token: {}", e)))?;

    let user = state
        .user_repo
        .get_user_by_id(Uuid::from_str(token_data.claims.sub.as_str()).unwrap())
        .await?;

    Ok(Json(user.to_safe()))
}
