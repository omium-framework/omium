//! Locate the `Omium` monorepo and path dependencies for generated apps.

use std::path::{Component, Path, PathBuf};

use anyhow::{Context, Result, bail};

/// Resolves a user-supplied project path against the current working directory.
pub fn resolve_project_path(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().context("failed to read current directory")?;
    Ok(cwd.join(path))
}

/// Finds the directory that contains `crates/omium-core`.
pub fn find_framework_root(start: &Path) -> Option<PathBuf> {
    let start = start.canonicalize().ok()?;
    let mut dir = start;
    loop {
        if dir.join("crates/omium-core").is_dir() {
            return Some(dir);
        }
        dir = dir.parent()?.to_path_buf();
    }
}

/// Relative path from `app_dir` to the framework `crates/` directory (for `Cargo.toml`).
pub fn detect_crate_path(app_dir: &Path) -> Result<String> {
    let app_dir = app_dir
        .canonicalize()
        .with_context(|| format!("application path not found: {}", app_dir.display()))?;

    let framework_root = find_framework_root(&app_dir).ok_or_else(|| {
        anyhow::anyhow!(
            "could not locate framework crates/ directory from {}. \
             Run `./omnisan new` from the Omium repository (e.g. examples/my-app).",
            app_dir.display()
        )
    })?;

    let crates_dir = framework_root.join("crates");
    relative_path(&app_dir, &crates_dir)
}

fn relative_path(from: &Path, to: &Path) -> Result<String> {
    let from = from
        .canonicalize()
        .with_context(|| format!("invalid path {}", from.display()))?;
    let to = to
        .canonicalize()
        .with_context(|| format!("invalid path {}", to.display()))?;

    let from_parts: Vec<_> = from.components().collect();
    let to_parts: Vec<_> = to.components().collect();

    let mut shared = 0usize;
    while shared < from_parts.len()
        && shared < to_parts.len()
        && from_parts[shared] == to_parts[shared]
    {
        shared += 1;
    }

    let mut rel = PathBuf::new();
    for _ in shared..from_parts.len() {
        rel.push("..");
    }
    for part in &to_parts[shared..] {
        match part {
            Component::Normal(name) => rel.push(name),
            Component::CurDir => {}
            _ => bail!("unsupported path component in {}", to.display()),
        }
    }

    rel.to_str()
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("non-utf8 relative path"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn relative_crates_from_nested_examples_app() {
        let temp = tempdir().expect("tempdir");
        std::fs::create_dir_all(temp.path().join("crates/omium-core")).expect("core");
        let app = temp.path().join("examples/demo-app");
        std::fs::create_dir_all(&app).expect("app dir");

        let rel = detect_crate_path(&app).expect("detect");
        assert_eq!(rel, "../../crates");
    }
}
