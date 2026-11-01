//! Authentication helpers: passwords, sessions, CSRF (phased), API tokens (phased).

mod csrf;
mod error;
mod password;
mod reset;
pub mod token;

pub use csrf::{ensure_token, peek_token, regenerate_token, verify_token};

pub mod session;

pub use error::{PasswordError, SessionError};
pub use password::{hash_password, verify_password};
pub use reset::{FileEmailVerificationStore, FilePasswordResetStore};
pub use session::{
    DatabaseSessionStore, FileSessionStore, Session, SessionCookieConfig, SessionHandle,
    SessionId, SessionStore, now_unix,
};
pub use token::{
    generate_plain_token, hash_api_token, parse_bearer_authorization, ApiTokenUser,
    ConfigTokenStore, DatabasePersonalAccessTokenStore, DisabledTokenStore,
    PersonalAccessTokenStore, TokenError, demo_token_store,
};
