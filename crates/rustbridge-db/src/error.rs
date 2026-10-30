//! Database adapter errors.

use thiserror::Error;

/// Errors while connecting to or using the database.
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database is disabled in configuration")]
    Disabled,

    #[error("database password is not configured")]
    MissingPassword,

    #[error("failed to connect to database: {0}")]
    Connection(#[from] sea_orm::DbErr),

    #[error("failed to run migrations: {0}")]
    Migration(String),

    #[error("transaction failed: {0}")]
    Transaction(String),
}
