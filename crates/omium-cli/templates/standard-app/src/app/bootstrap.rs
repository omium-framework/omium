//! Startup helpers (database; Vite constants kept for a shared app layout).

use omium_core::config::AppConfig;
use omium_db::{connect, ping};
use sea_orm::DatabaseConnection;

use super::RuntimeError;

pub const ASSET_VERSION: &str = "omium-standard-1";
pub const VITE_ENTRY: &str = "resources/js/app.ts";

/// Opens a database connection when `database.enabled` is true.
pub async fn connect_if_enabled(
    config: &AppConfig,
) -> Result<Option<DatabaseConnection>, RuntimeError> {
    if !config.database.enabled {
        tracing::info!("database disabled; enable it in config when you add models");
        return Ok(None);
    }

    let connection = match connect(&config.database).await {
        Ok(connection) => connection,
        Err(error) => {
            tracing::warn!(
                error = %error,
                "database connection failed; continuing without DB (start MySQL or set database.enabled = false)"
            );
            return Ok(None);
        }
    };

    if let Err(error) = ping(&connection).await {
        tracing::warn!(
            error = %error,
            "database ping failed; continuing without DB"
        );
        return Ok(None);
    }

    tracing::info!(
        database.host = %config.database.host,
        database.name = %config.database.name,
        "database connectivity verified"
    );
    Ok(Some(connection))
}

/// Not used in standard (Askama) mode; kept for shared `app` module shape.
pub fn resolve_vite_tags(_config: &AppConfig) -> Result<String, RuntimeError> {
    Ok(String::new())
}
