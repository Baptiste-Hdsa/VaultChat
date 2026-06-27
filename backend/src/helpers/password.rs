use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

/// Hashes a cleartext password into an Argon2id string
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

/// Checks if a cleartext password is correct against the known hash
pub fn verify_password(stored_hash: &str, provided_password: &str) -> bool {
    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    let argon2 = Argon2::default();

    argon2
        .verify_password(provided_password.as_bytes(), &parsed_hash)
        .is_ok()
}
