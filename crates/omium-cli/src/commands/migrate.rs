//! `migrate` — apply `SeaORM` migrations (Artisan `migrate` equivalent).

use clap::Args;

use super::project;

#[derive(Args, Debug)]
pub struct MigrateArgs {
    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: std::path::PathBuf,
}

/// Applies pending database migrations for the application in `path`.
///
/// # Errors
///
/// Returns an error when the migration crate is missing or Cargo fails.
pub async fn migrate(args: MigrateArgs) -> anyhow::Result<()> {
    let root = args
        .path
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("application path not found: {err}"))?;
    project::cargo_run_migration(&root).await
}
