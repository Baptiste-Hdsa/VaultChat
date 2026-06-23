use leptos::logging::log;

use crate::data_models::user::User;

pub fn login(pseudo: &str, password: &str) -> Option<User> {
    log!(
        "Calling backend to login pseudo: {} and password {}",
        pseudo,
        password
    );
    Some(User {
        id: "test".to_string(),
        pseudo: pseudo.to_string(),
    })
}
