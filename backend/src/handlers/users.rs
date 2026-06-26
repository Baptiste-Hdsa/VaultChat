// src/handlers/users.rs
// HTTP request handlers for users operations

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::user::{CreateUser, UpdateUserExtern, UpdateUserIntern, User};
use crate::error::{AppError, AppResult};
use crate::state::VaultChatState;

// GET /users - List users
pub async fn list_users(
    State(state): State<VaultChatState>,
) -> AppResult<Json<Vec<User>>> {
    let users = state.user_repo.get_all_users().await?;
    Ok(Json(users))
}

// GET /user/:id - Get an user by id
pub async fn get_user_by_id(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = state.user_repo.get_user_by_id(id).await?;
    Ok(Json(user))
}

// POST /users - Create a new user
pub async fn create_user(
    State(state): State<VaultChatState>,
    Json(input): Json<CreateUser>,
) -> AppResult<(StatusCode, Json<User>)> {
    if input.pseudo.trim().is_empty() {
        return Err(AppError::Validation("pseudo cannot be empty".to_string()));
    }

    if input.password.trim().is_empty() {
        return Err(AppError::Validation("Password cannot be empty".to_string()));
    }

    let user = state.user_repo.create_user(input).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

// PATCH /users/:id - Update an user
#[axum::debug_handler]
pub async fn update_user(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateUserExtern>,
) -> AppResult<Json<User>> {
    let new_user = UpdateUserIntern {
        id,
        pseudo: input.pseudo,
        password: input.password,
    };

    let user = state.user_repo.update_user(new_user).await?;

    Ok(Json(user))
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