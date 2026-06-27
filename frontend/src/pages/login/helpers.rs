use serde::Serialize;

use crate::{data_models::user::User, services::web::base_url};

#[derive(Serialize)]
struct LoginPayload {
    username: String,
    password: String,
}

pub async fn login_request(pseudo: &str, password: &str) -> Option<User> {
    let payload = LoginPayload {
        username: pseudo.to_string(),
        password: password.to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(base_url() + "/api/login")
        .json(&payload)
        .send()
        .await
        .ok()?;
    if response.status().is_success() {
        response.json::<User>().await.ok()
    } else {
        None
    }
}
