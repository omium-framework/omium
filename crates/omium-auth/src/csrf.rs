//! CSRF synchronizer token stored in the session (Laravel-style).

use argon2::password_hash::rand_core::{OsRng, RngCore};

use crate::session::Session;

const SESSION_KEY: &str = "_token";

/// Ensures a CSRF token exists in the session and returns it.
pub fn ensure_token(session: &mut Session) -> String {
    if let Some(existing) = session.get::<String>(SESSION_KEY) {
        return existing;
    }
    let token = generate_token();
    session.put(SESSION_KEY, token.clone());
    token
}

/// Returns the current token without creating one.
#[must_use]
pub fn peek_token(session: &Session) -> Option<String> {
    session.get(SESSION_KEY)
}

/// Validates a submitted token against the session value (constant-time compare).
#[must_use]
pub fn verify_token(session: &Session, submitted: &str) -> bool {
    if submitted.is_empty() {
        return false;
    }
    let Some(stored) = peek_token(session) else {
        return false;
    };
    constant_time_eq(stored.as_bytes(), submitted.as_bytes())
}

/// Rotates the CSRF token (call after login).
pub fn regenerate_token(session: &mut Session) -> String {
    let token = generate_token();
    session.put(SESSION_KEY, token.clone());
    token
}

fn generate_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0_u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::Session;

    #[test]
    fn ensure_and_verify() {
        let mut session = Session::new();
        let token = ensure_token(&mut session);
        assert!(verify_token(&session, &token));
        assert!(!verify_token(&session, "wrong"));
    }
}
