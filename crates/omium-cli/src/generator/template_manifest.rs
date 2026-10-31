//! Rename `*.toml.template` manifests after copying app scaffolds.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

const MANIFEST_TEMPLATES: &[(&str, &str)] = &[
    ("Cargo.toml.template", "Cargo.toml"),
    ("migration/Cargo.toml.template", "migration/Cargo.toml"),
];

/// Activates copied `Cargo.toml.template` files into real manifests.
///
/// Template sources use the `.template` suffix so `cargo package` on `omium-cli` does not
/// treat nested app scaffolds as separate crates (and exclude them from the published crate).
pub fn activate_manifest_templates(root: &Path) -> Result<()> {
    for (from, to) in MANIFEST_TEMPLATES {
        let source = root.join(from);
        if !source.is_file() {
            continue;
        }
        let target = root.join(to);
        if target.exists() {
            fs::remove_file(&target)
                .with_context(|| format!("remove existing {}", target.display()))?;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create directory {}", parent.display()))?;
        }
        fs::rename(&source, &target)
            .with_context(|| format!("activate manifest {}", target.display()))?;
    }
    Ok(())
}
