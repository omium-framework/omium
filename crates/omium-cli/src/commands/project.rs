//! Shared helpers for CLI subprocess commands.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use anyhow::{Context, Result, bail};
use tokio::process::Command;

/// Returns the current working directory as the application project root.
#[allow(dead_code)]
pub fn project_root() -> Result<PathBuf> {
    std::env::current_dir().context("failed to read current directory")
}

/// Returns true when the directory looks like a `Omium` application crate.
pub fn looks_like_app_crate(root: &Path) -> bool {
    root.join("Cargo.toml").is_file() && root.join("config").is_dir()
}

/// # Errors
///
/// Returns an error when `Cargo.toml` or `config/` is missing, with a short diagnosis.
pub fn ensure_app_crate(root: &Path) -> Result<()> {
    if looks_like_app_crate(root) {
        return Ok(());
    }
    bail!(
        "not in an application directory ({}): {}",
        app_crate_diagnosis(root),
        root.display()
    );
}

fn app_crate_diagnosis(root: &Path) -> String {
    let mut issues = Vec::new();
    if !root.join("Cargo.toml").is_file() {
        issues.push("missing Cargo.toml");
    }
    if !root.join("config").is_dir() {
        issues.push("missing config/");
    }
    let mut message = issues.join(", ");
    if issues.len() == 2 && root.join("omnisan").is_file() {
        message.push_str(
            " — folder looks like an incomplete `./omnisan new` run; \
             generate into a new directory (e.g. examples/my-app) with \
             `./omnisan new examples/my-app --frontend vue --yes`",
        );
    }
    message
}

/// Returns true when the project includes a Vite/npm frontend toolchain.
pub fn has_vite_frontend(root: &Path) -> bool {
    root.join("package.json").is_file() && root.join("vite.config.ts").is_file()
}

/// `[package].name` from the application `Cargo.toml` (used as the primary `[[bin]]` name).
///
/// # Errors
///
/// Returns an error when the manifest is missing or has no package name.
pub fn app_package_name(root: &Path) -> Result<String> {
    let manifest = root.join("Cargo.toml");
    let contents = std::fs::read_to_string(&manifest)
        .with_context(|| format!("read {}", manifest.display()))?;
    parse_package_name(&contents).with_context(|| format!("parse {}", manifest.display()))
}

fn parse_package_name(cargo_toml: &str) -> Result<String> {
    let mut in_package = false;
    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package = true;
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed != "[package]" {
            if in_package {
                break;
            }
            in_package = false;
        }
        if !in_package {
            continue;
        }
        let Some(rest) = trimmed.strip_prefix("name") else {
            continue;
        };
        let rest = rest.trim();
        let Some((_, value)) = rest.split_once('=') else {
            continue;
        };
        let value = value.trim().trim_matches('"').trim_matches('\'');
        if value.is_empty() {
            bail!("empty package name in [package]");
        }
        return Ok(value.to_string());
    }
    bail!("no [package] name in Cargo.toml")
}

fn app_has_route_list_binary(root: &Path) -> bool {
    root.join("src/bin/route_list.rs").is_file()
}

/// Builds the `cargo run …` argument list for the application binary.
fn cargo_run_argv(root: &Path, extra_args: &[&str]) -> Result<Vec<String>> {
    let mut run_args: Vec<String> = Vec::new();
    if app_has_route_list_binary(root) {
        let bin = app_package_name(root)?;
        run_args.push("--bin".to_string());
        run_args.push(bin);
    }
    run_args.extend(extra_args.iter().map(|s| (*s).to_string()));
    Ok(run_args)
}

/// `cargo watch` subcommand for `./omnisan dev` (recompile + restart on `src/` changes).
pub fn cargo_watch_installed() -> bool {
    std::process::Command::new("cargo")
        .args(["watch", "--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Installs `cargo-watch` when missing (one-time per machine; used by `./omnisan dev`).
///
/// # Errors
///
/// Returns an error when `cargo install` fails or `cargo watch` stays unavailable.
pub async fn ensure_cargo_watch() -> Result<()> {
    if cargo_watch_installed() {
        return Ok(());
    }

    println!("Installing cargo-watch for Rust hot reload (one-time; may take a minute)…");

    let status = Command::new("cargo")
        .args(["install", "cargo-watch"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context("failed to spawn cargo install cargo-watch")?;

    if !status.success() {
        bail!(
            "cargo install cargo-watch failed with status {status}. \
             Check network access and retry, or use `./omnisan dev --no-rust-watch`."
        );
    }

    if !cargo_watch_installed() {
        bail!(
            "cargo-watch was installed but `cargo watch` is not on PATH. \
             Add ~/.cargo/bin to your shell PATH, then retry."
        );
    }

    println!("OK  cargo-watch ready.");
    Ok(())
}

/// Path to the optional `SeaORM` migration manifest for this app.
pub fn migration_manifest(root: &Path) -> PathBuf {
    root.join("migration/Cargo.toml")
}

/// Ensures `.env` exists by copying `.env.example` when missing.
pub fn ensure_local_env_file(root: &Path) -> Result<()> {
    let env_path = root.join(".env");
    if env_path.is_file() {
        return Ok(());
    }
    let example = root.join(".env.example");
    if !example.is_file() {
        return Ok(());
    }
    std::fs::copy(&example, &env_path)
        .with_context(|| format!("copy {} -> {}", example.display(), env_path.display()))?;
    println!(
        "Created {} from .env.example (edit locally; never commit .env).",
        env_path.display()
    );
    Ok(())
}
///
/// # Errors
///
/// Returns an error when Cargo is missing or exits with a non-zero status.
pub async fn cargo_run_in(root: &Path, extra_args: &[&str]) -> Result<()> {
    ensure_local_env_file(root)?;
    cargo_run_in_with_env(root, extra_args, &[]).await
}

/// Runs `cargo run` with additional environment variables for the child process.
///
/// # Errors
///
/// Returns an error when Cargo is missing or exits with a non-zero status.
pub async fn cargo_run_in_with_env(
    root: &Path,
    extra_args: &[&str],
    env_pairs: &[(&str, &str)],
) -> Result<()> {
    ensure_app_crate(root)?;

    let run_args = cargo_run_argv(root, extra_args)?;

    let mut command = Command::new("cargo");
    command
        .arg("run")
        .args(&run_args)
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    for (key, value) in env_pairs {
        command.env(key, value);
    }

    let status = command
        .status()
        .await
        .context("failed to spawn cargo run")?;
    if !status.success() {
        bail!("cargo run failed with status {status}");
    }
    Ok(())
}

/// Runs `cargo watch -x run …` so Rust sources reload without restarting `./omnisan dev`.
///
/// # Errors
///
/// Returns an error when `cargo-watch` is missing or the watch process fails.
pub async fn cargo_watch_run_in_with_env(
    root: &Path,
    extra_args: &[&str],
    env_pairs: &[(&str, &str)],
) -> Result<()> {
    ensure_app_crate(root)?;

    ensure_cargo_watch().await?;

    let run_args = cargo_run_argv(root, extra_args)?;
    let mut run_spec = String::from("run");
    for arg in &run_args {
        run_spec.push(' ');
        run_spec.push_str(arg);
    }

    let mut command = Command::new("cargo");
    command
        .args(["watch", "-c", "-x", &run_spec])
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    for (key, value) in env_pairs {
        command.env(key, value);
    }

    tracing::info!("Rust: cargo watch (restarts server when src/ changes)");

    let status = command
        .status()
        .await
        .context("failed to spawn cargo watch")?;
    if !status.success() {
        bail!("cargo watch failed with status {status}");
    }
    Ok(())
}

/// Runs `cargo run --manifest-path migration/Cargo.toml`.
///
/// # Errors
///
/// Returns an error when no migration crate exists or Cargo fails.
pub async fn cargo_run_migration(root: &Path) -> Result<()> {
    ensure_local_env_file(root)?;
    let manifest = migration_manifest(root);
    if !manifest.is_file() {
        bail!(
            "no migration crate at {} — generate a Vue/standard app or add migration/",
            manifest.display()
        );
    }

    let status = Command::new("cargo")
        .args([
            "run",
            "--manifest-path",
            manifest.to_str().context("non-utf8 path")?,
        ])
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context("failed to spawn migration")?;

    if !status.success() {
        bail!("migration command failed with status {status}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_name_reads_first_package_section() {
        let toml = r#"
[package]
name = "om-test"
version = "0.0.0"

[[bin]]
name = "route-list"
"#;
        assert_eq!(
            parse_package_name(toml).expect("name"),
            "om-test".to_string()
        );
    }

    #[test]
    fn cargo_run_argv_adds_bin_when_route_list_exists() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        std::fs::write(root.join("Cargo.toml"), "[package]\nname = \"demo-app\"\n").expect("cargo");
        std::fs::create_dir_all(root.join("src/bin")).expect("dir");
        std::fs::write(root.join("src/bin/route_list.rs"), "fn main() {}").expect("bin");
        let argv = cargo_run_argv(root, &[]).expect("argv");
        assert_eq!(argv, vec!["--bin", "demo-app"]);
    }
}

/// Runs `npm install` in the project directory.
///
/// # Errors
///
/// Returns an error when npm is missing or install fails.
pub async fn npm_install(root: &Path) -> Result<()> {
    if !root.join("package.json").is_file() {
        bail!("no package.json in {}", root.display());
    }

    let status = Command::new("npm")
        .args(["install"])
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context("failed to spawn npm install")?;

    if !status.success() {
        bail!("npm install failed with status {status}");
    }
    Ok(())
}

/// Ensures `node_modules` exists (runs `npm install` when missing).
///
/// # Errors
///
/// Returns an error when npm install fails.
pub async fn ensure_npm_dependencies(root: &Path) -> Result<()> {
    if !root.join("package.json").is_file() {
        return Ok(());
    }
    if root.join("node_modules").is_dir() {
        return Ok(());
    }
    npm_install(root).await
}

/// Waits until a TCP port accepts connections (Vite dev server ready).
///
/// # Errors
///
/// Returns an error when the port does not become ready in time.
pub async fn wait_for_port(host: &str, port: u16, attempts: u32) -> Result<()> {
    use tokio::net::TcpStream;
    use tokio::time::{Duration, sleep};

    let addr = format!("{host}:{port}");
    for attempt in 1..=attempts {
        if TcpStream::connect(&addr).await.is_ok() {
            return Ok(());
        }
        if attempt < attempts {
            sleep(Duration::from_millis(250)).await;
        }
    }
    bail!("timed out waiting for {addr} (is Vite running? try `./omnisan install` first)")
}

/// Runs `npm run build` when `package.json` exists.
///
/// # Errors
///
/// Returns an error when npm is missing or the build fails.
pub async fn npm_run_build(root: &Path) -> Result<()> {
    if !root.join("package.json").is_file() {
        bail!("no package.json in {} — nothing to build", root.display());
    }

    let status = Command::new("npm")
        .args(["run", "build"])
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .context("failed to spawn npm run build")?;

    if !status.success() {
        bail!("npm run build failed with status {status}");
    }
    Ok(())
}

/// Spawns `npm run dev` and returns the child handle (killed when dropped).
///
/// # Errors
///
/// Returns an error when npm cannot be started.
pub fn spawn_vite_dev(root: &Path) -> Result<tokio::process::Child> {
    if !has_vite_frontend(root) {
        bail!(
            "no Vite frontend detected in {} (expected package.json and vite.config.ts)",
            root.display()
        );
    }

    Command::new("npm")
        .args(["run", "dev"])
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .context("failed to spawn npm run dev")
}
