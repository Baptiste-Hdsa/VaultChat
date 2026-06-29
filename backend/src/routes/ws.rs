use axum::{Router, routing::get};

use crate::{state::VaultChatState, websocket::ws_setup::websocket_handler};

pub fn ws_routes() -> Router<VaultChatState> {
    Router::new().route("/ws", get(websocket_handler))
}
