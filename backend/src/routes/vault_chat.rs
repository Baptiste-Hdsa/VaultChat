use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::vault_chat::{
    create_chat_message, delete_chat_message, get_message_by_id, list_chat_messages,
    update_chat_message, VaultChatState,
};

pub fn vault_chat_routes() -> Router<VaultChatState> {
    Router::new()
        .route("/chats/:sender_id/:receiver_id/messages", get(list_chat_messages))
        .route("/messages", post(create_chat_message))
        .route("/messages/:id", get(get_message_by_id))
        .route("/messages/:id", patch(update_chat_message))
        .route("/messages/:id", delete(delete_chat_message))
}