use leptos::logging::log;

pub fn login(pseudo: &str, password: &str) {
    log!(
        "Calling backend to login pseudo: {} and password {}",
        pseudo,
        password
    );
}
