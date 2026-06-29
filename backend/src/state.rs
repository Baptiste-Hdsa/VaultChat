use tokio::sync::broadcast;

// src/state.rs
use crate::db::messages::MessageRepository;
use crate::db::users::UserRepository;

// VaultChatState holds shared application state
// Clone is cheap because PgPool uses Arc internally
#[derive(Clone)]
pub struct VaultChatState {
    pub message_repo: MessageRepository,
    pub user_repo: UserRepository,
    pub tx: broadcast::Sender<String>, // Web sockets
}
