//! Application bootstrap: configuration, tracing, and shared state.

use std::path::Path;

use crate::config::{AppConfig, load_config};
use crate::error::BootstrapError;
use crate::state::AppState;
use crate::telemetry::init_tracing;

/// Options for bootstrapping an application process.
#[derive(Debug, Clone)]
pub struct BootstrapOptions {
    pub config_dir: std::path::PathBuf,
}

impl BootstrapOptions {
    #[must_use]
    pub fn new(config_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            config_dir: config_dir.into(),
        }
    }
}

/// Loads configuration, initializes tracing, and builds [`AppState`].
///
/// # Errors
///
/// Returns [`BootstrapError`] when configuration or tracing initialization fails.
pub fn bootstrap(options: &BootstrapOptions) -> Result<AppState, BootstrapError> {
    let config = load_config(&options.config_dir)?;
    init_tracing(&config.logging)?;
    log_startup(&config);
    Ok(AppState::new(config))
}

fn log_startup(config: &AppConfig) {
    tracing::info!(
        app.name = %config.app.name,
        app.environment = %config.app.environment,
        app.debug = config.app.debug,
        logging.level = %config.logging.level,
        database.enabled = config.database.enabled,
        config = ?config,
        "application bootstrapped"
    );
}

/// Bootstraps the application and waits for a shutdown signal.
///
/// # Errors
///
/// Returns [`BootstrapError`] when bootstrap fails.
pub async fn bootstrap_and_wait_for_shutdown(
    options: BootstrapOptions,
) -> Result<(), BootstrapError> {
    let state = bootstrap(&options)?;
    tracing::info!(
        app.name = %state.config().app.name,
        "sample runtime idle until shutdown signal"
    );
    crate::shutdown::wait_for_shutdown_signal().await;
    tracing::info!("application shutdown complete");
    Ok(())
}

/// Resolves a configuration directory relative to the current working directory.
#[must_use]
pub fn resolve_config_dir(path: Option<&Path>) -> std::path::PathBuf {
    path.map_or_else(
        || std::path::PathBuf::from("config"),
        std::path::Path::to_path_buf,
    )
}
