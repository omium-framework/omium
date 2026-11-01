//! `dev` — Vite dev server + Rust HTTP server (local full-stack workflow).

use clap::Args;

use super::project;

#[derive(Args, Debug, Clone)]
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
        let port = read_server_port(&root).unwrap_or(3002);
        let rust_args = args.clone();
        let rust_root = root.clone();
        let rust_handle = tokio::spawn(async move { run_rust_app(&rust_args, &rust_root).await });

        tracing::info!(
            port,
            "Waiting for Rust HTTP server (first compile can take a minute)…"
        );
        project::wait_for_port("127.0.0.1", port, 240)
            .await
            .map_err(|err| {
                anyhow::anyhow!(
                    "{err}. Inertia needs http://127.0.0.1:{port} — run `./omnisan dev`, not `npm run dev` alone."
                )
            })?;
        tracing::info!(
            app_url = %format!("http://127.0.0.1:{port}"),
            "Rust HTTP server ready — open this URL in the browser (Inertia + session)"
        );
        tracing::info!(
            vite = %args.vite_origin.trim(),
            "Starting Vite for HMR assets only (:5173 is not the app entry)"
        );

        let _vite = project::spawn_vite_dev(&root)?;
        project::wait_for_port("127.0.0.1", 5173, 120).await?;
        tracing::info!(
            origin = %args.vite_origin.trim(),
            "Vite dev server ready"
        );

        rust_handle.await??;
        Ok(())
    } else {
        run_rust_app(&args, &root).await
    }
}

fn read_server_port(root: &std::path::Path) -> Option<u16> {
    let app_toml = std::fs::read_to_string(root.join("config/app.toml")).ok()?;
    let server_toml = std::fs::read_to_string(root.join("config/server.toml")).ok();
    let parse_port = |contents: &str| -> Option<u16> {
        contents
            .lines()
            .find_map(|line| {
                let line = line.trim();
                line.strip_prefix("port = ")
                    .and_then(|value| value.parse().ok())
            })
    };
    parse_port(&app_toml).or_else(|| server_toml.as_deref().and_then(parse_port))
}
