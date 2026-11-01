//! Startup helpers (database, Vite asset tags).

use std::path::PathBuf;

use omium_core::config::AppConfig;
use omium_db::{connect, ping};
use omium_inertia::inertia_rs::{vite_dev_tags, vite_production_tags};
use sea_orm::DatabaseConnection;

use super::RuntimeError;

pub const ASSET_VERSION: &str = "omium-react-1";
pub const VITE_ENTRY: &str = "resources/js/app.tsx";

/// Opens a database connection when `database.enabled` is true.
///
/// # Errors
///
/// Returns [`RuntimeError`] when connection or ping fails.
pub async fn connect_if_enabled(
    config: &AppConfig,
) -> Result<Option<DatabaseConnection>, RuntimeError> {
    if !config.database.enabled {
        tracing::info!("database disabled; Examples page will show setup instructions");
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

/// Resolves HTML shell script tags for Vite dev or production manifest.
///
/// # Errors
///
/// Returns [`RuntimeError`] when the production manifest cannot be read.
pub fn resolve_vite_tags(_config: &AppConfig) -> Result<String, RuntimeError> {
    if let Ok(origin) = std::env::var("RUSTBRIDGE_VITE_DEV")
        && !origin.trim().is_empty()
    {
        tracing::info!(%origin, "using Vite dev server tags");
        return Ok(vite_dev_tags(origin.trim(), VITE_ENTRY));
    }

    let manifest =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public/build/.vite/manifest.json");
    if manifest.is_file() {
        tracing::info!(path = %manifest.display(), "using Vite production manifest");
        return Ok(vite_production_tags(&manifest, VITE_ENTRY)?);
    }

    tracing::warn!(
        "no Vite dev env and no public/build/.vite/manifest.json — using placeholder script tag"
    );
    Ok(String::from(
        "<!-- Run ./omnisan install && ./omnisan dev (Vite hot reload) -->",
    ))
}
