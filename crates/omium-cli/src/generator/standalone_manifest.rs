//! Pin `[workspace]` placeholders for apps generated outside the Framework monorepo.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

const WORKSPACE_TO_PINNED: &[(&str, &str)] = &[
    ("axum = { workspace = true }", "axum = \"0.8.8\""),
    (
        "sea-orm = { workspace = true }",
        r#"sea-orm = { version = "1.1.19", default-features = false, features = ["macros", "runtime-tokio-rustls", "sqlx-mysql", "with-chrono"] }"#,
    ),
    (
        "serde = { workspace = true }",
        "serde = { version = \"1.0.228\", features = [\"derive\"] }",
    ),
    (
        "serde_json = { workspace = true }",
        "serde_json = \"1.0.145\"",
    ),
    ("thiserror = { workspace = true }", "thiserror = \"2.0.17\""),
    (
        "tokio = { workspace = true }",
        "tokio = { version = \"1.49.0\", features = [\"macros\", \"rt-multi-thread\", \"signal\"] }",
    ),
    (
        "tower-http = { workspace = true, features = [\"fs\"] }",
        "tower-http = { version = \"0.6.8\", features = [\"fs\"] }",
    ),
    ("tracing = { workspace = true }", "tracing = \"0.1.44\""),
    ("tower = { workspace = true }", "tower = \"0.5.2\""),
    (
        "async-trait = { workspace = true }",
        "async-trait = \"0.1.89\"",
    ),
    (
        "sea-orm-migration = { workspace = true }",
        r#"sea-orm-migration = { version = "1.1.19", default-features = false, features = ["runtime-tokio-rustls", "sqlx-mysql"] }"#,
    ),
];

/// Replaces `edition.workspace` / `{ workspace = true }` for standalone crates.io apps.
pub fn finalize_standalone_manifest(manifest: &Path) -> Result<()> {
    let mut content =
        fs::read_to_string(manifest).with_context(|| format!("read {}", manifest.display()))?;

    content = content.replace("edition.workspace = true", "edition = \"2024\"");
    content = content.replace("[lints]\nworkspace = true\n\n", "");
    content = content.replace("[lints]\nworkspace = true\n", "");

    for (from, to) in WORKSPACE_TO_PINNED {
        content = content.replace(from, to);
    }

    fs::write(manifest, content).with_context(|| format!("write {}", manifest.display()))?;
    Ok(())
}

/// Makes a generated app build with `cargo`/`omnisan dev` inside a parent repo (e.g. `Framework/examples/*`).
///
/// Cargo otherwise treats the app as an orphaned workspace member when `edition.workspace` or parent
/// `Cargo.toml` exists above the app directory.
pub fn ensure_standalone_package(manifest: &Path) -> Result<()> {
    finalize_standalone_manifest(manifest)?;

    let content =
        fs::read_to_string(manifest).with_context(|| format!("read {}", manifest.display()))?;
    if content.contains("members = ") {
        return Ok(());
    }
    if content.lines().any(|line| line.trim() == "[workspace]") {
        return Ok(());
    }

    let updated = format!(
        "{content}\n# Generated app — not a member of the Framework workspace.\n[workspace]\n"
    );
    fs::write(manifest, updated).with_context(|| format!("write {}", manifest.display()))?;
    Ok(())
}

/// Root manifest for apps that include a `migration/` crate (virtual workspace).
pub fn ensure_standalone_app_workspace(app_manifest: &Path) -> Result<()> {
    finalize_standalone_manifest(app_manifest)?;

    let migration_dir = app_manifest
        .parent()
        .map(|p| p.join("migration"))
        .filter(|p| p.is_dir());
    let Some(migration_dir) = migration_dir else {
        return ensure_standalone_package(app_manifest);
    };

    let migration_manifest = migration_dir.join("Cargo.toml");
    if migration_manifest.is_file() {
        finalize_standalone_manifest(&migration_manifest)?;
        strip_workspace_table(&migration_manifest)?;
    }

    let mut content = fs::read_to_string(app_manifest)
        .with_context(|| format!("read {}", app_manifest.display()))?;
    if content.contains("members = ") {
        return Ok(());
    }

    content.push_str(
        r#"
# Generated app — not a member of the Framework workspace.
[workspace]
members = [".", "migration"]
resolver = "2"
"#,
    );
    fs::write(app_manifest, content)
        .with_context(|| format!("write {}", app_manifest.display()))?;
    Ok(())
}

fn strip_workspace_table(manifest: &Path) -> Result<()> {
    let content =
        fs::read_to_string(manifest).with_context(|| format!("read {}", manifest.display()))?;
    let Some(idx) = content
        .lines()
        .position(|line| line.trim() == "[workspace]")
    else {
        return Ok(());
    };
    let kept: Vec<&str> = content
        .lines()
        .take(idx)
        .filter(|line| !line.starts_with("# Generated app"))
        .collect();
    let mut trimmed = kept.join("\n");
    trimmed = trimmed.trim_end().to_string();
    fs::write(manifest, format!("{trimmed}\n"))
        .with_context(|| format!("write {}", manifest.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn adds_workspace_table_for_nested_apps() {
        let mut file = NamedTempFile::new().expect("temp");
        write!(
            file,
            "[package]\nname = \"demo\"\nedition = \"2024\"\n\n[dependencies]\n"
        )
        .expect("write");
        ensure_standalone_package(file.path()).expect("ensure");
        let out = std::fs::read_to_string(file.path()).expect("read");
        assert!(out.contains("[workspace]"));
    }

    #[test]
    fn strips_workspace_edition_and_lints() {
        let mut file = NamedTempFile::new().expect("temp");
        write!(
            file,
            "[package]\nedition.workspace = true\n\n[lints]\nworkspace = true\n\n[dependencies]\naxum = {{ workspace = true }}\n"
        )
        .expect("write");
        finalize_standalone_manifest(file.path()).expect("patch");
        let out = std::fs::read_to_string(file.path()).expect("read");
        assert!(out.contains("edition = \"2024\""));
        assert!(!out.contains("workspace = true"));
        assert!(out.contains("axum = \"0.8.8\""));
    }
}
