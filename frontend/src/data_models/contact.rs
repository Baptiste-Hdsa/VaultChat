use crate::data_models::message::Message;

#[derive(Clone, Debug)]
pub struct Contact {
    pub pseudo: String,
    pub id: String,
    pub last_message: Message,
}
