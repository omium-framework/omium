//! Password hashing and verification (Phase A1).

mod error;
mod password;

pub use error::PasswordError;
pub use password::{hash_password, verify_password};
