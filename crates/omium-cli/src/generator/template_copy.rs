//! Copy template trees into generated app directories.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

pub const COPY_SKIP_DIRS: &[&str] = &["node_modules", "target", ".git", "public/build"];

pub fn copy_tree(src: &Path, dst: &Path, force: bool, skip_dirs: &[&str]) -> Result<()> {
    if dst.exists() {
        let non_empty = fs::read_dir(dst)?.next().is_some();
        if non_empty && !force {
            bail!(
                "refusing to overwrite non-empty directory: {} (use --yes)",
                dst.display()
            );
        }
    } else {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if skip_dirs.iter().any(|skip| *skip == name_str) {
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        if from.is_dir() {
            copy_tree(&from, &to, true, skip_dirs)?;
        } else if !to.exists() || force {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)
                .with_context(|| format!("copy {} -> {}", from.display(), to.display()))?;
        }
    }
    Ok(())
}

pub fn replace_in_file(path: &Path, from: &str, to: &str) -> Result<()> {
    if !path.is_file() {
        return Ok(());
    }
    let contents = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    if !contents.contains(from) {
        return Ok(());
    }
    fs::write(path, contents.replace(from, to))
        .with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
