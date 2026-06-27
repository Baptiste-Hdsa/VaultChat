use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

// What gets stored securely inside the token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // The user's ID
    pub exp: usize,  // Expiration timestamp
}

pub const JWT_SECRET: &[u8] = b"gougougagak-1234-nothing-matters";

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(2))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}
