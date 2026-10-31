//! Opaque session identifier (cookie value).

use argon2::password_hash::rand_core::{OsRng, RngCore};

use crate::SessionError;

/// Session id stored in the session cookie (hex-encoded random bytes).
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SessionId(String);

impl SessionId {
    /// Creates a new cryptographically random session id.
    #[must_use]
    pub fn generate() -> Self {
        let mut bytes = [0_u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Self(hex_encode(&bytes))
    }

    /// Parses an existing id from the cookie; rejects path traversal characters.
    pub fn parse(raw: &str) -> Result<Self, SessionError> {
        if raw.is_empty() || raw.len() > 128 {
            return Err(SessionError::InvalidSessionId);
        }
        if !raw.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(SessionError::InvalidSessionId);
        }
        Ok(Self(raw.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_ids_are_alphanumeric() {
        let id = SessionId::generate();
        assert!(SessionId::parse(id.as_str()).is_ok());
    }

    #[test]
    fn rejects_invalid_ids() {
        assert!(SessionId::parse("../etc/passwd").is_err());
        assert!(SessionId::parse("").is_err());
    }
}
