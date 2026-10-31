//! Argon2id password hashing (PHC string format).

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::PasswordError;

/// Hashes a plaintext password for storage (Argon2id, random salt).
///
/// # Errors
///
/// Returns [`PasswordError::EmptyPassword`] when `plain` is empty.
pub fn hash_password(plain: &str) -> Result<String, PasswordError> {
    if plain.is_empty() {
        return Err(PasswordError::EmptyPassword);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|_| PasswordError::HashFailed)?;
    Ok(hash.to_string())
}

/// Verifies a plaintext password against a PHC hash string.
///
/// # Errors
///
/// Returns [`PasswordError::InvalidHash`] when the stored hash cannot be parsed.
pub fn verify_password(plain: &str, stored_hash: &str) -> Result<bool, PasswordError> {
    if plain.is_empty() {
        return Ok(false);
    }

    let parsed = PasswordHash::new(stored_hash).map_err(|_| PasswordError::InvalidHash)?;
    Ok(Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_round_trip() {
        let hash = hash_password("correct-horse-battery-staple").expect("hash");
        assert!(hash.starts_with("$argon2"));
        assert!(verify_password("correct-horse-battery-staple", &hash).expect("verify"));
        assert!(!verify_password("wrong", &hash).expect("verify wrong"));
    }

    #[test]
    fn empty_password_is_rejected_for_hash() {
        assert!(matches!(
            hash_password(""),
            Err(PasswordError::EmptyPassword)
        ));
    }

    #[test]
    fn invalid_stored_hash_returns_error() {
        assert!(matches!(
            verify_password("x", "not-a-valid-phc-string"),
            Err(PasswordError::InvalidHash)
        ));
    }
}
