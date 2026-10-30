//! `rustbridge doctor` command.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Args;
use rustbridge_core::config::load_config;
use rustbridge_db::{connect, ping};

#[derive(Args, Debug)]
pub struct DoctorArgs {
    #[arg(long, default_value = "config")]
    pub config_dir: PathBuf,
}

/// Runs local environment and configuration checks.
pub fn doctor(args: &DoctorArgs) -> Result<()> {
    if !args.config_dir.is_dir() {
        bail!(
            "config directory not found: {} (run from an application root or pass --config-dir)",
            args.config_dir.display()
        );
    }

    let config = load_config(&args.config_dir).context("configuration failed to load")?;
    println!("OK  configuration loaded");
    println!("    app.name = {}", config.app.name);
    println!("    app.environment = {}", config.app.environment);
    println!("    server = {}:{}", config.server.host, config.server.port);

    if config.database.enabled {
        println!("OK  database.enabled = true");
        let rt = tokio::runtime::Runtime::new().context("failed to start Tokio runtime")?;
        rt.block_on(async {
            let connection = connect(&config.database)
                .await
                .context("database connection failed")?;
            ping(&connection).await.context("database ping failed")?;
            Ok::<(), anyhow::Error>(())
        })?;
        println!("OK  database connectivity verified");
    } else {
        println!("..  database.enabled = false (skipped connectivity check)");
    }

    println!("Doctor checks completed successfully.");
    Ok(())
}
