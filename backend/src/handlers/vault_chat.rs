// src/handlers/vault_chat.rs
// HTTP request handlers for message operations

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::vault_chat::{UpdateMessageExtern, UpdateMessageIntern};
use crate::models::{
    user::Contact,
    vault_chat::{CreateMessage, Message},
};
use crate::state::VaultChatState;

// GET /chats/:sender_id/:receiver_id/messages - List chat messages
pub async fn list_chat_messages(
    State(state): State<VaultChatState>,
    Path((sender_id, receiver_id)): Path<(Uuid, Uuid)>,
) -> AppResult<Json<Vec<Message>>> {
    let messages = state
        .message_repo
        .list_chat_messages(sender_id, receiver_id)
        .await?;
    Ok(Json(messages))
}

// GET /messages/:id - Get a single message by id
pub async fn get_message_by_id(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Message>> {
    let message = state.message_repo.get_message_by_id(id).await?;
    Ok(Json(message))
}

// POST /messages - Create a new message
pub async fn create_chat_message(
    State(state): State<VaultChatState>,
    Json(input): Json<CreateMessage>,
) -> AppResult<(StatusCode, Json<Message>)> {
    if let Some(content) = &input.receiver_content {
        if content.trim().is_empty() {
            return Err(AppError::Validation("Message cannot be empty".to_string()));
        }
    }

    let message = if input.receiver_id.is_some() {
        state.message_repo.create_message(input).await?
    } else {
        return Err(AppError::Validation(
            "Message cannot be send to no one".to_string(),
        ));
    };

    let _ = state.tx.send("NEW_MESSAGE".to_string());
    Ok((StatusCode::CREATED, Json(message)))
}

// PATCH /messages/:id - Update a message
pub async fn update_chat_message(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateMessageExtern>,
) -> AppResult<Json<Message>> {
    let new_message = UpdateMessageIntern {
        id,
        content: input.content.clone(),
    };
    if let Some(ref message) = input.content {
        if message.trim().is_empty() {
            return Err(AppError::Validation("Message cannot be empty".to_string()));
        }
    }

    let message = state.message_repo.update_message(new_message).await?;
    Ok(Json(message))
}

// DELETE /messages/:id - Delete a message
pub async fn delete_chat_message(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let deleted = state.message_repo.delete_message(id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!(
            "Message with id {} not found",
            id
        )))
    }
}

pub async fn get_user_contacts(
    State(state): State<VaultChatState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Vec<Contact>>> {
    let mut contacts = vec![];

    let self_last_message = match state.message_repo.get_last_message(id, id).await {
        Ok(last_message) => Some(last_message),
        Err(_) => None,
    };
    contacts.push(
        state
            .user_repo
            .get_user_by_id(id)
            .await?
            .to_safe()
            .to_contact(self_last_message),
    );

    let users = state.message_repo.get_user_contacts(id).await?;
    for user in users {
        let last_message = match state.message_repo.get_last_message(id, user.id).await {
            Ok(last_message) => Some(last_message),
            Err(_) => None,
        };
        contacts.push(user.to_contact(last_message));
    }
    Ok(Json(contacts))
}
