//! `rustbridge new` command.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, ValueEnum};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FrontendMode {
    Api,
    Standard,
    Vue,
    React,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Directory name for the generated application
    pub path: PathBuf,

    #[arg(long, default_value = "Omium")]
    pub app_name: String,

    #[arg(long, value_enum, default_value_t = FrontendMode::Api)]
    pub frontend: FrontendMode,

    /// Overwrite existing files without confirmation
    #[arg(long)]
    pub yes: bool,
}

/// Generates a new application skeleton.
pub fn new_app(args: &NewArgs) -> Result<()> {
    if args.path.exists() {
        if !args.path.is_dir() {
            bail!("target path exists and is not a directory");
        }
        if fs::read_dir(&args.path)?.next().is_some() && !args.yes {
            bail!(
                "target directory is not empty: {} (use --yes to continue anyway)",
                args.path.display()
            );
        }
    } else {
        fs::create_dir_all(&args.path)
            .with_context(|| format!("failed to create directory {}", args.path.display()))?;
    }

    match args.frontend {
        FrontendMode::Api => generate_api_app(&args.path, &args.app_name, args.yes)?,
        FrontendMode::Standard | FrontendMode::Vue | FrontendMode::React => {
            bail!(
                "frontend mode {:?} is not implemented yet; use --frontend api for now",
                args.frontend
            );
        }
    }

    println!("Generated API application at {}", args.path.display());
    println!("Next steps:");
    println!("  cd {}", args.path.display());
    println!("  cargo run -p rustbridge-cli -- serve --config-dir config");
    Ok(())
}

fn generate_api_app(path: &Path, app_name: &str, force: bool) -> Result<()> {
    let crate_path = detect_crate_path(path)?;
    write_file(
        path,
        "Cargo.toml",
        &api_cargo_toml(&slug(path), &crate_path),
        force,
    )?;
    write_file(path, "config/app.toml", &app_toml(app_name), force)?;
    write_file(path, "config/logging.toml", logging_toml(), force)?;
    write_file(path, "config/server.toml", server_toml(), force)?;
    write_file(path, "config/database.toml", database_toml(), force)?;
    write_file(path, "src/main.rs", api_main_rs(), force)?;
    write_file(path, "README.md", &api_readme(app_name), force)?;
    Ok(())
}

fn slug(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("app")
        .replace('-', "_")
}

fn write_file(base: &Path, relative: &str, contents: &str, force: bool) -> Result<()> {
    let target = base.join(relative);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    if target.exists() && !force {
        bail!("refusing to overwrite existing file: {}", target.display());
    }
    fs::write(&target, contents)
        .with_context(|| format!("failed to write {}", target.display()))?;
    Ok(())
}

fn detect_crate_path(app_dir: &Path) -> Result<String> {
    let candidates = ["../crates", "../../crates", "../../../crates"];
    for prefix in candidates {
        if app_dir.join(prefix).join("rustbridge-core").is_dir() {
            return Ok(prefix.to_string());
        }
    }
    bail!(
        "could not locate framework crates/ directory; run `rustbridge new` from the repository root or examples/"
    );
}

fn api_cargo_toml(crate_name: &str, crate_path: &str) -> String {
    format!(
        r#"[package]
name = "{crate_name}"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
rustbridge-core = {{ path = "{crate_path}/rustbridge-core" }}
rustbridge-http = {{ path = "{crate_path}/rustbridge-http" }}
tokio = {{ version = "1", features = ["macros", "rt-multi-thread", "signal"] }}
"#
    )
}

fn app_toml(app_name: &str) -> String {
    format!(
        r#"[app]
name = "{app_name}"
environment = "local"
debug = true
"#
    )
}

fn logging_toml() -> &'static str {
    r#"[logging]
level = "info"
"#
}

fn server_toml() -> &'static str {
    r#"[server]
host = "127.0.0.1"
port = 8080
request_timeout_secs = 30
body_limit_bytes = 2097152
"#
}

fn database_toml() -> &'static str {
    r#"[database]
enabled = false
host = "127.0.0.1"
port = 3306
name = "omium"
username = "omium"
"#
}

fn api_main_rs() -> &'static str {
    r#"use std::path::PathBuf;

use rustbridge_core::bootstrap::{BootstrapOptions, bootstrap};
use rustbridge_http::serve;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = PathBuf::from("config");
    let state = bootstrap(&BootstrapOptions::new(config_dir))?;
    serve(state).await?;
    Ok(())
}
"#
}

fn api_readme(app_name: &str) -> String {
    format!(
        "# {app_name}\n\n\
         Generated RustBridge API application skeleton.\n\n\
         ```bash\n\
         cargo run\n\
         curl http://127.0.0.1:8080/health\n\
         ```\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_api_app_tree() {
        let temp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(temp.path().join("crates/rustbridge-core")).expect("crates");
        new_app(&NewArgs {
            path: temp.path().join("demo-api"),
            app_name: String::from("Omium"),
            frontend: FrontendMode::Api,
            yes: true,
        })
        .expect("generate");
        assert!(temp.path().join("demo-api/src/main.rs").is_file());
        assert!(temp.path().join("demo-api/config/app.toml").is_file());
    }
}
