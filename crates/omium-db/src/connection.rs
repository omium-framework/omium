//! `MySQL` connection pool setup.

use std::time::Duration;

use omium_core::config::DatabaseSettings;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::error::DatabaseError;

const DEFAULT_MAX_CONNECTIONS: u32 = 10;
const DEFAULT_MIN_CONNECTIONS: u32 = 1;
const CONNECT_TIMEOUT_SECS: u64 = 8;

/// Builds a `MySQL` connection URL from typed settings (password is URL-encoded).
///
/// # Errors
///
/// Returns [`DatabaseError::MissingPassword`] when no password is configured.
pub fn build_database_url(settings: &DatabaseSettings) -> Result<String, DatabaseError> {
    let password = settings
        .password
        .as_deref()
        .filter(|value| !value.is_empty())
        .ok_or(DatabaseError::MissingPassword)?;

    Ok(format!(
        "mysql://{}:{}@{}:{}/{}",
        urlencoding::encode(&settings.username),
        urlencoding::encode(password),
        settings.host,
        settings.port,
        settings.name
    ))
}

/// Connects to `MySQL` using framework database settings.
///
/// # Errors
///
/// Returns [`DatabaseError`] when settings are invalid or the server is unreachable.
pub async fn connect(settings: &DatabaseSettings) -> Result<DatabaseConnection, DatabaseError> {
    if !settings.enabled {
        return Err(DatabaseError::Disabled);
    }

    let url = build_database_url(settings)?;
    let mut options = ConnectOptions::new(url);
    options
        .max_connections(DEFAULT_MAX_CONNECTIONS)
        .min_connections(DEFAULT_MIN_CONNECTIONS)
        .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS))
        .sqlx_logging(false);

    Database::connect(options)
        .await
        .map_err(DatabaseError::Connection)
}

/// Runs a lightweight connectivity check (`SELECT 1`).
///
/// # Errors
///
/// Returns [`DatabaseError`] when the query fails.
pub async fn ping(connection: &DatabaseConnection) -> Result<(), DatabaseError> {
    use sea_orm::ConnectionTrait;

    connection
        .query_one(sea_orm::Statement::from_string(
            sea_orm::DatabaseBackend::MySql,
            String::from("SELECT 1"),
        ))
        .await
        .map_err(DatabaseError::Connection)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_url_encodes_credentials() {
        let settings = DatabaseSettings {
            enabled: true,
            host: String::from("127.0.0.1"),
            port: 3306,
            name: String::from("omium"),
            username: String::from("omium"),
            password: Some(String::from("p@ss w/rd")),
        };

        let url = build_database_url(&settings).expect("url");
        assert!(url.contains("p%40ss"));
        assert!(url.contains("w%2Frd"));
    }
}
