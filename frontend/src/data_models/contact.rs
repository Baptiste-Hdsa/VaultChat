use serde::Deserialize;

use crate::data_models::message::Message;

#[derive(Clone, Debug, Deserialize)]
pub struct Contact {
    pub id: String,
    pub username: String,
    pub public_key: String,
    pub last_message: Option<Message>,
}
