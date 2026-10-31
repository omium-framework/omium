//! `run` — start the application HTTP server (`cargo run` wrapper).

use clap::Args;

use super::project;

#[derive(Args, Debug)]
pub struct RunArgs {
    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: std::path::PathBuf,
}

/// Runs the application binary via Cargo from the project directory.
///
/// # Errors
///
/// Returns an error when the directory is not an app or Cargo fails.
pub async fn run_app(args: RunArgs) -> anyhow::Result<()> {
    let root = args
        .path
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("application path not found: {err}"))?;
    project::cargo_run_in(&root, &[]).await
}
