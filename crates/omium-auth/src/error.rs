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
