// src/handlers/users.rs
// HTTP request handlers for users operations

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::models::user::CreateUser;
use crate::models::user::{UpdateUserExtern, UpdateUserIntern};
use crate::state::VaultChatState;
use crate::{
    error::{AppError, AppResult},
    models::user::SafeUser,
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

    let response = state.user_repo.create_user(input).await?;

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
    Json(input): Json<CreateUser>,
) -> AppResult<Json<SafeUser>> {
    let user = state.user_repo.get_user_by_username(input.username).await?;

    if user.password.eq(&input.password) {
        return Ok(Json(user.to_safe()));
    }
    Err(AppError::NotFound(format!("Pseudo or password incorrect")))
}
