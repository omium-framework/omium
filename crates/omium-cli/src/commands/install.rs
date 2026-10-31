//! `install` — project setup (`npm install`, local `.env`, dev tooling).

use clap::Args;

use super::project;

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: std::path::PathBuf,

    /// Skip npm even when `package.json` exists
    #[arg(long)]
    pub skip_frontend: bool,

    /// Do not create `.env` from `.env.example`
    #[arg(long)]
    pub skip_env: bool,
}

/// Installs dependencies and prepares local environment files.
///
/// # Errors
///
/// Returns an error when npm or `cargo install cargo-watch` fails.
pub async fn install(args: InstallArgs) -> anyhow::Result<()> {
    let root = args
        .path
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("application path not found: {err}"))?;

    if !args.skip_env {
        project::ensure_local_env_file(&root)?;
    }

    if args.skip_frontend {
        println!("Frontend install skipped (--skip-frontend).");
    } else if !root.join("package.json").is_file() {
        println!(
            "No package.json in {} — frontend install skipped.",
            root.display()
        );
    } else {
        project::ensure_npm_dependencies(&root).await?;
        println!("Frontend dependencies installed.");
    }

    project::ensure_cargo_watch().await?;
    Ok(())
}
