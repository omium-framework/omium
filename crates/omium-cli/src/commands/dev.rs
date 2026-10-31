//! `dev` — Vite dev server + Rust HTTP server (local full-stack workflow).

use clap::Args;

use super::project;

#[derive(Args, Debug)]
pub struct DevArgs {
    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: std::path::PathBuf,

    /// Vite dev server origin passed to `RUSTBRIDGE_VITE_DEV`
    #[arg(long, default_value = "http://127.0.0.1:5173")]
    pub vite_origin: String,

    /// Do not spawn `npm run dev` (Rust only, production manifest or placeholder tags)
    #[arg(long)]
    pub rust_only: bool,

    /// Single `cargo run` (no `cargo watch`); restart manually after Rust changes
    #[arg(long)]
    pub no_rust_watch: bool,
}

async fn run_rust_app(args: &DevArgs, root: &std::path::Path) -> anyhow::Result<()> {
    let env = [("RUSTBRIDGE_VITE_DEV", args.vite_origin.trim())];
    if args.no_rust_watch {
        if args.rust_only {
            project::cargo_run_in(root, &[]).await
        } else {
            project::cargo_run_in_with_env(root, &[], &env).await
        }
    } else if args.rust_only {
        project::cargo_watch_run_in_with_env(root, &[], &[]).await
    } else {
        project::cargo_watch_run_in_with_env(root, &[], &env).await
    }
}

/// Starts Vite (when present) and runs the application with `cargo watch` + `cargo run`.
///
/// # Errors
///
/// Returns an error when npm or Cargo fails.
pub async fn dev(args: DevArgs) -> anyhow::Result<()> {
    let root = args
        .path
        .canonicalize()
        .map_err(|err| anyhow::anyhow!("application path not found: {err}"))?;

    if !args.rust_only && project::has_vite_frontend(&root) {
        project::ensure_npm_dependencies(&root).await?;
        let _vite = project::spawn_vite_dev(&root)?;
        project::wait_for_port("127.0.0.1", 5173, 120).await?;
        tracing::info!(
            origin = %args.vite_origin.trim(),
            "Vite dev server ready (Vue HMR via @vite/client)"
        );
        run_rust_app(&args, &root).await
    } else {
        run_rust_app(&args, &root).await
    }
}
