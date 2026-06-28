use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub public_key: String,
    pub wrapped_private_key: String,
    pub crypto_salt: String,
    pub aes_iv: String,
}
