//! `build` — compile frontend assets (Vite) and optionally the Rust crate.

use clap::Args;

use super::project;

#[derive(Args, Debug)]
pub struct BuildArgs {
    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: std::path::PathBuf,

    /// Skip `npm run build` even when a Vite project is present
    #[arg(long)]
    pub skip_frontend: bool,

    /// Also run `cargo build --release`
    #[arg(long)]
    pub release: bool,
}

/// Builds frontend assets and optionally the Rust binary.
///
/// # Errors
///
/// Returns an error when npm or Cargo fails.
pub async fn build(args: BuildArgs) -> anyhow::Result<()> {
    let root = args
        .path
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("application path not found: {err}"))?;

    if !args.skip_frontend && project::has_vite_frontend(&root) {
        project::ensure_npm_dependencies(&root).await?;
        project::npm_run_build(&root).await?;
        println!(
            "Frontend build complete. Restart the HTTP server to load new assets, or use `./omnisan dev` for hot reload."
        );
    }

    if args.release {
        let status = tokio::process::Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&root)
            .status()
            .await?;
        if !status.success() {
            anyhow::bail!("cargo build --release failed");
        }
    }

    Ok(())
}
