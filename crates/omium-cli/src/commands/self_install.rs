//! `self-install` — install the `omnisan` binary into `~/.cargo/bin`.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::generator::framework_paths;

/// Installs this CLI with `cargo install` so you can run `omnisan` without `./omnisan`.
///
/// # Errors
///
/// Returns an error when the framework root or `cargo install` cannot be resolved.
pub fn self_install() -> Result<()> {
    let start = std::env::current_dir().context("failed to read current directory")?;
    let framework_root = framework_paths::find_framework_root(&start).ok_or_else(|| {
        anyhow::anyhow!(
            "could not find the Omium repository from {}.\n\
             Run this from the Framework checkout or an app inside it, or run manually:\n\
             cargo install --path /path/to/Framework/crates/omium-cli --bin omnisan",
            start.display()
        )
    })?;

    let manifest = framework_root.join("crates/omium-cli/Cargo.toml");
    if !manifest.is_file() {
        bail!(
            "CLI manifest missing at {} — is this a Omium checkout?",
            manifest.display()
        );
    }

    let manifest_arg = manifest
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("non-utf8 path: {}", manifest.display()))?;

    println!("Installing omnisan from {} …", framework_root.display());

    let status = Command::new("cargo")
        .args([
            "install",
            "--path",
            manifest_arg,
            "--bin",
            "omnisan",
            "--locked",
        ])
        .status()
        .context("failed to run cargo install")?;

    if !status.success() {
        bail!("cargo install failed with status {status}");
    }

    print_success_hints();
    Ok(())
}

fn print_success_hints() {
    let cargo_bin = cargo_home_bin_dir();
    println!();
    println!("Done. The `omnisan` command should be on your PATH if `~/.cargo/bin` is configured.");
    if let Some(dir) = cargo_bin {
        println!("Binary directory: {}", dir.display());
    }
    println!();
    println!("Try:");
    println!("  omnisan --version");
    println!("  cd examples/inertia-vue-app && omnisan doctor");
    println!();
    println!("You can keep using `./omnisan` in projects; it will delegate to the global binary.");
}

fn cargo_home_bin_dir() -> Option<PathBuf> {
    std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")))
        .map(|home| home.join("bin"))
}
