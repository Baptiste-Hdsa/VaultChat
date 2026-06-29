use axum::extract::State;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::ws::WebSocketUpgrade;

use crate::state::VaultChatState;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<VaultChatState>,
) -> impl axum::response::IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: VaultChatState) {
    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}
