use std::time::Instant;

use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
