//! Password reset and email verification tokens (file-backed for templates).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{SessionError, now_unix};
use crate::token::hash_api_token;

const RESET_TTL_SECS: i64 = 60 * 60;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResetRecord {
    email: String,
    expires_at: i64,
}

/// Stores password reset tokens on disk (`storage/framework/password_reset`).
#[derive(Clone, Debug)]
pub struct FilePasswordResetStore {
    directory: PathBuf,
}

impl FilePasswordResetStore {
    #[must_use]
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    /// Creates a reset token for `email` (returns plaintext token for the email link).
    pub fn issue(&self, email: &str) -> Result<String, SessionError> {
        fs::create_dir_all(&self.directory)?;
        let plain = crate::generate_plain_token();
        let record = ResetRecord {
            email: email.to_string(),
            expires_at: now_unix() + RESET_TTL_SECS,
        };
        let path = self.directory.join(format!("{}.json", hash_api_token(&plain)));
        let payload = serde_json::to_string(&record)?;
        fs::write(path, payload)?;
        Ok(plain)
    }

    /// Validates and consumes a reset token, returning the email if valid.
    pub fn consume(&self, plain_token: &str) -> Result<Option<String>, SessionError> {
        let path = self.directory.join(format!("{}.json", hash_api_token(plain_token)));
        if !path.is_file() {
            return Ok(None);
        }
        let contents = fs::read_to_string(&path)?;
        let record: ResetRecord = serde_json::from_str(&contents)?;
        let _ = fs::remove_file(&path);
        if record.expires_at < now_unix() {
            return Ok(None);
        }
        Ok(Some(record.email))
    }
}

/// Marks an email as verified (demo until `users.email_verified_at` in A7).
#[derive(Clone, Debug)]
pub struct FileEmailVerificationStore {
    directory: PathBuf,
}

impl FileEmailVerificationStore {
    #[must_use]
    pub fn new(directory: impl Into<PathBuf>) -> Self {
        Self {
            directory: directory.into(),
        }
    }

    pub fn issue(&self, email: &str) -> Result<String, SessionError> {
        fs::create_dir_all(&self.directory)?;
        let plain = crate::generate_plain_token();
        let path = self.directory.join(format!("{}.json", hash_api_token(&plain)));
        let payload = serde_json::to_string(&VerificationRecord {
            email: email.to_string(),
            expires_at: now_unix() + RESET_TTL_SECS,
        })?;
        fs::write(path, payload)?;
        Ok(plain)
    }

    pub fn verify(&self, plain_token: &str) -> Result<Option<String>, SessionError> {
        let path = self.directory.join(format!("{}.json", hash_api_token(plain_token)));
        if !path.is_file() {
            return Ok(None);
        }
        let contents = fs::read_to_string(&path)?;
        let record: VerificationRecord = serde_json::from_str(&contents)?;
        let _ = fs::remove_file(&path);
        if record.expires_at < now_unix() {
            return Ok(None);
        }
        mark_verified(&self.directory, &record.email)?;
        Ok(Some(record.email))
    }

    pub fn is_verified(&self, email: &str) -> Result<bool, SessionError> {
        Ok(verified_marker_path(&self.directory, email).is_file())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct VerificationRecord {
    email: String,
    expires_at: i64,
}

fn mark_verified(base: &Path, email: &str) -> Result<(), SessionError> {
    fs::create_dir_all(base.join("verified"))?;
    let marker = verified_marker_path(base, email);
    fs::write(marker, email.as_bytes())?;
    Ok(())
}

fn verified_marker_path(base: &Path, email: &str) -> PathBuf {
    base.join("verified").join(format!("{}.verified", hash_api_token(email)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_token_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = FilePasswordResetStore::new(dir.path());
        let token = store.issue("user@example.com").expect("issue");
        let email = store.consume(&token).expect("consume");
        assert_eq!(email.as_deref(), Some("user@example.com"));
        assert_eq!(store.consume(&token).expect("reuse"), None);
    }
}
