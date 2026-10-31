use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password must not be empty")]
    EmptyPassword,
    #[error("stored password hash is invalid")]
    InvalidHash,
    #[error("failed to hash password")]
    HashFailed,
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("invalid session id")]
    InvalidSessionId,
    #[error("session store I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("session serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("database session error: {0}")]
    Database(String),
}
