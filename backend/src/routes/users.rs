use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::users::{create_user, delete_user, get_user_by_id, list_users, update_user};
use crate::state::VaultChatState;

pub fn user_routes() -> Router<VaultChatState> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users", post(create_user))
        .route("/users/:id", patch(update_user))
        .route("/users/:id", delete(delete_user))
}