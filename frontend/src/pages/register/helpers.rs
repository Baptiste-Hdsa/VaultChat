use leptos::{logging::log, prelude::*};
use zxcvbn::{Score, zxcvbn};

use unicode_normalization::UnicodeNormalization;

const ILLEGAL_TEXTS: &[&str] = &["vaultchat"];

pub struct CredentialErr {
    pseudo_errors: Vec<String>,
    password_errors: Vec<String>,
    password_mismatch: bool,
}

impl CredentialErr {
    pub fn get_pseudo_errors(&self) -> Vec<String> {
        return self.pseudo_errors.clone();
    }
    pub fn get_password_errors(&self) -> Vec<String> {
        return self.password_errors.clone();
    }
    pub fn are_passwords_different(&self) -> bool {
        return self.password_mismatch;
    }
}

fn normalize_string(text: &str) -> String {
    let normalized = text.nfc().collect::<String>();
    normalized
}

fn check_pseudo(pseudo: &str) -> Vec<String> {
    let mut errors: Vec<String> = vec![];
    let normalized_pseudo = normalize_string(pseudo);
    if normalized_pseudo.trim().eq("") {
        errors.push("Pseudo is mandatory".to_string());
        return errors;
    }

    log!("Ask db if pseudo: {} is used", pseudo);

    return errors;
}

fn check_password_strength(pseudo: &str, password: &str) -> Vec<String> {
    let mut errors: Vec<String> = vec![];
    if password.is_empty() {
        return errors;
    }

    let normalized_password = normalize_string(password);
    let lower_password = normalized_password.to_lowercase(); // used to check for known illegal text
    let normalized_pseudo = normalize_string(pseudo).trim().to_lowercase();

    let estimate = zxcvbn(password, &[pseudo]);

    if estimate.score() < Score::Three {
        if let Some(feedback) = estimate.feedback() {
            if let Some(warning) = feedback.warning() {
                errors.push(warning.to_string());
            }
            for suggestion in feedback.suggestions() {
                errors.push(suggestion.to_string());
            }
        }

        if errors.is_empty() {
            errors.push("Password is too guessable. Please add uncommon words.".to_string());
        }
    };

    if normalized_password.len() < 16 {
        errors.push("Should be at least 16 characters long".to_string());
    }

    for illegal_text in ILLEGAL_TEXTS {
        if lower_password.contains(illegal_text) {
            errors.push(format!("Password cannot contain the text {illegal_text}"));
        }
    }

    if !normalized_pseudo.is_empty() && lower_password.contains(&normalized_pseudo) {
        errors.push(format!(
            "Password cannot contain your own pseudo (\"{}\")",
            normalized_pseudo
        ));
    }

    return errors;
}

pub fn register(pseudo: &str, password: &str) {
    log!(
        "Calling backend to register pseudo: {} and password {}",
        pseudo,
        password
    );
}

pub fn check_credentials(
    pseudo: &str,
    password: &str,
    confirm_password: &str,
) -> Result<(), CredentialErr> {
    let pseudo_errors = check_pseudo(pseudo);
    let password_errors = check_password_strength(pseudo, password);
    let password_mismatch = !confirm_password.is_empty() && password != confirm_password;
    if password_mismatch || !password_errors.is_empty() || !pseudo_errors.is_empty() {
        return Err(CredentialErr {
            pseudo_errors,
            password_errors: password_errors,
            password_mismatch: password_mismatch,
        });
    }
    return Ok(());
}

pub fn update_errors(
    errors_holder: Result<(), CredentialErr>,
    set_pseudo_errors: WriteSignal<Vec<String>>,
    set_password_errors: WriteSignal<Vec<String>>,
    set_confirm_password_errors: WriteSignal<bool>,
) {
    match errors_holder {
        Ok(_) => {
            set_pseudo_errors.set(Vec::new());
            set_password_errors.set(Vec::new());
            set_confirm_password_errors.set(false);
        }
        Err(errors) => {
            set_pseudo_errors.set(errors.get_pseudo_errors());
            set_password_errors.set(errors.get_password_errors());
            set_confirm_password_errors.set(errors.are_passwords_different());
        }
    }
}

pub fn is_pseudo_taken(pseudo: String) -> bool {
    log!("TODO make backend call to check availability");
    return false;
}
