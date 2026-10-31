//! crates.io dependency lines for generated applications.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use super::framework_paths;

/// Published Omium crate version (keep in sync with `[workspace.package].version`).
pub const OMIUM_REGISTRY_VERSION: &str = "0.1.4";

/// How generated app `Cargo.toml` files resolve framework crates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyMode {
    /// Path dependencies into a cloned Framework `crates/` directory.
    Monorepo { crates_relative: String },
    /// Version pins from crates.io (`omium-*` packages and `omium_*` Rust crate names).
    Registry,
}

/// Chooses monorepo path deps when an ancestor contains `crates/omium-core`, else registry.
pub fn dependency_mode_for_app(app_dir: &Path) -> Result<DependencyMode> {
    if framework_paths::find_framework_root(app_dir).is_some() {
        let crates_relative = framework_paths::detect_crate_path(app_dir)?;
        return Ok(DependencyMode::Monorepo { crates_relative });
    }
    Ok(DependencyMode::Registry)
}

fn registry_line(folder: &str) -> String {
    match folder {
        "omium-core" | "omium-http" | "omium-db" | "omium-view" | "omium-inertia" => {}
        other => panic!("unknown framework folder {other}"),
    };
    format!(
        "{folder} = {{ version = \"{OMIUM_REGISTRY_VERSION}\" }}",
        folder = folder,
        OMIUM_REGISTRY_VERSION = OMIUM_REGISTRY_VERSION,
    )
}

fn path_line(folder: &str, crates_relative: &str) -> String {
    format!(
        "{folder} = {{ path = \"{crates_relative}/{folder}\" }}",
        folder = folder,
        crates_relative = crates_relative,
    )
}

fn inertia_registry_line() -> String {
    format!(
        "omium-inertia = {{ version = \"{OMIUM_REGISTRY_VERSION}\", features = [\"inertia-rs\"] }}"
    )
}

fn inertia_path_line(crates_relative: &str) -> String {
    format!(
        "omium-inertia = {{ path = \"{crates_relative}/omium-inertia\", features = [\"inertia-rs\"] }}",
        crates_relative = crates_relative,
    )
}

/// API app `[dependencies]` framework section.
pub fn api_framework_dependencies(mode: &DependencyMode) -> String {
    match mode {
        DependencyMode::Monorepo { crates_relative } => format!(
            "{}\n{}\n{}\n{}",
            path_line("omium-core", crates_relative),
            path_line("omium-http", crates_relative),
            path_line("omium-db", crates_relative),
            path_line("omium-view", crates_relative),
        ),
        DependencyMode::Registry => format!(
            "{}\n{}\n{}\n{}",
            registry_line("omium-core"),
            registry_line("omium-http"),
            registry_line("omium-db"),
            registry_line("omium-view"),
        ),
    }
}

/// Standard (Askama) app framework dependencies.
pub fn standard_framework_dependencies(mode: &DependencyMode) -> String {
    match mode {
        DependencyMode::Monorepo { crates_relative } => format!(
            "{}\n{}\n{}",
            path_line("omium-core", crates_relative),
            path_line("omium-http", crates_relative),
            path_line("omium-view", crates_relative),
        ),
        DependencyMode::Registry => format!(
            "{}\n{}\n{}",
            registry_line("omium-core"),
            registry_line("omium-http"),
            registry_line("omium-view"),
        ),
    }
}

/// Vue/React app root `Cargo.toml` framework dependencies block.
pub fn inertia_app_framework_dependencies(mode: &DependencyMode) -> String {
    match mode {
        DependencyMode::Monorepo { crates_relative } => format!(
            "{}\n{}\n{}\n{}\n{}",
            path_line("omium-core", crates_relative),
            path_line("omium-db", crates_relative),
            path_line("omium-http", crates_relative),
            inertia_path_line(crates_relative),
            path_line("omium-view", crates_relative),
        ),
        DependencyMode::Registry => format!(
            "{}\n{}\n{}\n{}\n{}",
            registry_line("omium-core"),
            registry_line("omium-db"),
            registry_line("omium-http"),
            inertia_registry_line(),
            registry_line("omium-view"),
        ),
    }
}

/// Migration crate framework dependencies.
pub fn migration_framework_dependencies(mode: &DependencyMode, app_dir: &Path) -> Result<String> {
    match mode {
        DependencyMode::Monorepo { .. } => {
            let migration_crates = framework_paths::detect_crate_path(&app_dir.join("migration"))?;
            Ok(format!(
                "{}\n{}",
                path_line("omium-core", &migration_crates),
                path_line("omium-db", &migration_crates),
            ))
        }
        DependencyMode::Registry => Ok(format!(
            "{}\n{}",
            registry_line("omium-core"),
            registry_line("omium-db"),
        )),
    }
}

/// Replaces template `path = "../../crates/..."` lines in a copied manifest.
pub fn rewrite_template_manifest(
    manifest: &Path,
    mode: &DependencyMode,
    app_dir: &Path,
) -> Result<()> {
    let content =
        fs::read_to_string(manifest).with_context(|| format!("read {}", manifest.display()))?;

    let framework_block = if manifest
        .file_name()
        .is_some_and(|name| name == "Cargo.toml")
        && manifest.parent().is_some_and(|p| p.ends_with("migration"))
    {
        migration_framework_dependencies(mode, app_dir)?
    } else if manifest
        .parent()
        .is_some_and(|p| p.join("migration").is_dir())
    {
        if content.contains("omium-inertia") {
            inertia_app_framework_dependencies(mode)
        } else {
            api_framework_dependencies(mode)
        }
    } else {
        return Ok(());
    };

    let updated = replace_framework_dependency_block(&content, &framework_block);
    fs::write(manifest, updated).with_context(|| format!("write {}", manifest.display()))?;
    Ok(())
}

fn replace_framework_dependency_block(content: &str, framework_block: &str) -> String {
    let mut out = Vec::new();
    let mut in_dependencies = false;
    let mut inserted_framework = false;
    let mut skipping_multiline = false;
    let mut brace_depth = 0i32;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed == "[dependencies]" {
            in_dependencies = true;
            out.push(line.to_string());
            continue;
        }

        if in_dependencies && trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_dependencies = false;
        }

        if in_dependencies {
            if skipping_multiline {
                brace_depth += trimmed.chars().filter(|c| *c == '{').count() as i32;
                brace_depth -= trimmed.chars().filter(|c| *c == '}').count() as i32;
                if brace_depth <= 0 {
                    skipping_multiline = false;
                }
                continue;
            }

            if let Some(key) = trimmed.split('=').next().map(str::trim) {
                if key.starts_with("omium-") {
                    if !inserted_framework {
                        out.extend(framework_block.lines().map(String::from));
                        inserted_framework = true;
                    }
                    brace_depth = trimmed.chars().filter(|c| *c == '{').count() as i32
                        - trimmed.chars().filter(|c| *c == '}').count() as i32;
                    if brace_depth > 0 {
                        skipping_multiline = true;
                    }
                    continue;
                }
            }
        }

        out.push(line.to_string());
    }

    out.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_api_deps_contain_omium_packages() {
        let block = api_framework_dependencies(&DependencyMode::Registry);
        assert!(block.contains("version = \"0.1.4\""));
    }

    #[test]
    fn rewrite_removes_omium_view_after_other_deps() {
        let sample = r#"[dependencies]
axum = { workspace = true }
omium-core = { path = "../../crates/omium-core" }
omium-inertia = { path = "../../crates/omium-inertia", features = [
    "inertia-rs",
] }
sea-orm = { workspace = true }
omium-view = { path = "../../crates/omium-view" }
"#;
        let block = inertia_app_framework_dependencies(&DependencyMode::Registry);
        let out = replace_framework_dependency_block(sample, &block);
        assert_eq!(out.matches("omium-view").count(), 1);
        assert!(out.contains("omium-core = { version = \"0.1.4\" }"));
    }
}
