//! Applies Vue app migrations using framework configuration.

use std::path::PathBuf;

use vue_inertia_app_migration::Migrator;
use omium_core::config::load_config;
use omium_db::{DatabaseError, apply_migrations, connect};

#[tokio::main]
async fn main() -> Result<(), MigrateError> {
    let config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../config");
    let config = load_config(&config_dir).map_err(MigrateError::Config)?;

    if !config.database.enabled {
        eprintln!(
            "database.enabled is false. Enable MySQL in config and set APP_DATABASE__PASSWORD."
        );
        return Err(MigrateError::DatabaseDisabled);
    }

    let connection = connect(&config.database).await?;
    apply_migrations::<Migrator>(&connection).await?;
    println!("Migrations applied successfully.");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum MigrateError {
    #[error(transparent)]
    Config(#[from] omium_core::error::ConfigError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error("database is disabled in configuration")]
    DatabaseDisabled,
}
