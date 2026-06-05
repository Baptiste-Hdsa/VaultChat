// src/db/pool.rs
// Database connection pool management

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

// Create a connection pool with production-ready settings
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .max_lifetime(Duration::from_secs(30 * 60))
        .idle_timeout(Duration::from_secs(10 * 60))
        .connect(database_url)
        .await
}