//! Personal access tokens (Sanctum-style): plain token once, SHA-256 hash at rest.

mod store;

use sha2::{Digest, Sha256};

pub use store::{
    ConfigTokenStore, DatabasePersonalAccessTokenStore, DisabledTokenStore,
    PersonalAccessTokenStore, TokenError, demo_token_store,
};

/// Authenticated API user attached to the request after Bearer validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ApiTokenUser {
    pub user_id: u64,
}

/// SHA-256 hex digest of the plaintext token (storage format).
#[must_use]
pub fn hash_api_token(plain: &str) -> String {
    let digest = Sha256::digest(plain.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Generates a new random plaintext token (64 hex chars).
#[must_use]
pub fn generate_plain_token() -> String {
    use password_hash::rand_core::{OsRng, RngCore};
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Parses `Authorization: Bearer {token}`.
#[must_use]
pub fn parse_bearer_authorization(header_value: &str) -> Option<&str> {
    let trimmed = header_value.trim();
    let rest = trimmed.strip_prefix("Bearer")?;
    let token = rest.trim_start();
    if token.is_empty() {
        None
    } else {
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_stable_hex() {
        let hash = hash_api_token("omium-dev-api-token");
        assert_eq!(hash.len(), 64);
        assert_eq!(hash, hash_api_token("omium-dev-api-token"));
    }

    #[test]
    fn bearer_parser_accepts_standard_header() {
        assert_eq!(
            parse_bearer_authorization("Bearer omium-dev-api-token"),
            Some("omium-dev-api-token")
        );
    }
}
