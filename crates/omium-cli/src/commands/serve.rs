//! `serve` — production-style HTTP server (`cargo run`, built Vite assets).

use clap::Args;
use omium_core::config::load_config;

use super::dev::DevArgs;
use super::{dev, project};

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Path to the configuration directory (informational; apps load `./config` today)
    #[arg(long, default_value = "config")]
    pub config_dir: std::path::PathBuf,

    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: std::path::PathBuf,

    /// Force production serve (skip auto-`dev` for Vue apps in debug mode)
    #[arg(long)]
    pub production: bool,
}

/// Starts the HTTP server. Vue apps in debug mode automatically run `dev` (Vite HMR).
///
/// # Errors
///
/// Returns an error when the project root is invalid or Cargo fails.
pub async fn serve(args: ServeArgs) -> Result<(), anyhow::Error> {
    let _ = args.config_dir;
    let root = args
        .path
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("application path not found: {err}"))?;

    if !args.production && project::has_vite_frontend(&root) {
        let config = load_config(root.join("config"))
            .map_err(|err| anyhow::anyhow!("failed to load config: {err}"))?;
        if config.app.debug {
            println!(
                "Debug Vue app: starting `./omnisan dev` (Vite hot reload). \
                 Pass --production to serve built assets from public/build instead."
            );
            return dev(DevArgs {
                path: root,
                vite_origin: String::from("http://127.0.0.1:5173"),
                rust_only: false,
                no_rust_watch: false,
            })
            .await;
        }
    }

    project::cargo_run_in(&root, &[]).await
}
