//! `rustbridge serve` command.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use rustbridge_core::bootstrap::{BootstrapOptions, bootstrap};
use rustbridge_db::{connect, ping};
use rustbridge_http::serve as http_serve;

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Path to the configuration directory (contains app.toml)
    #[arg(long, default_value = "config")]
    pub config_dir: PathBuf,
}

/// Starts the application HTTP server from the current project directory.
///
/// # Errors
///
/// Returns an error when bootstrap, database connectivity, or HTTP serving fails.
pub async fn serve(args: ServeArgs) -> Result<()> {
    let config_dir = args
        .config_dir
        .canonicalize()
        .with_context(|| format!("config directory not found: {}", args.config_dir.display()))?;

    let state = bootstrap(&BootstrapOptions::new(config_dir.clone()))
        .context("failed to bootstrap application")?;

    if state.config().database.enabled {
        let connection = connect(&state.config().database)
            .await
            .context("failed to connect to database")?;
        ping(&connection).await.context("database ping failed")?;
        tracing::info!("database connectivity verified");
    } else {
        tracing::info!("database disabled in configuration");
    }

    tracing::info!(
        config_dir = %config_dir.display(),
        "starting HTTP server (use sample-app for home page with DB listing)"
    );
    http_serve(state).await.context("HTTP server failed")?;
    Ok(())
}
