// src/main.rs
// Application entry point
// https://oneuptime.com/blog/post/2026-01-26-rust-backend-development/view#setting-up-your-development-environment

use axum::Router;
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod error;
mod handlers;
mod helpers;
mod models;
mod routes;
mod security;
mod websocket;

pub mod state;

use db::messages::MessageRepository;
use db::pool::create_pool;
use db::users::UserRepository;
use routes::users::user_routes;
use routes::vault_chat::vault_chat_routes;
use routes::ws::ws_routes;
use state::VaultChatState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for structured logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vaultchat=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create database connection pool
    tracing::info!("Connecting to database...");
    let pool = create_pool(&database_url).await?;
    tracing::info!("Database connection established");

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Migrations complete");

    let (tx, _rx) = broadcast::channel(100); // Web sockets
    // Create application state
    let state = VaultChatState {
        message_repo: MessageRepository::new(pool.clone()),
        user_repo: UserRepository::new(pool),
        tx,
    };

    // Build the router
    let app = Router::new()
        .merge(vault_chat_routes())
        .merge(user_routes())
        .merge(ws_routes())
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
