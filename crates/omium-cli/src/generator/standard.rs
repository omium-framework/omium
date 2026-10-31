//! `omnisan new --frontend standard` — unified Laravel-style app tree (Askama HTML).

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use super::omnisan_script::write_app_omnisan_wrapper;
use crate::generator::new::write_env_example;
use crate::generator::template_copy::{copy_tree, replace_in_file, COPY_SKIP_DIRS};

const TEMPLATE_PACKAGE: &str = "omium-standard-app";
const TEMPLATE_LIB: &str = "omium_standard_app";
const TEMPLATE_MIGRATION_PACKAGE: &str = "omium-standard-app-migration";
const TEMPLATE_MIGRATION_LIB: &str = "omium_standard_app_migration";

pub fn generate_standard_app(dest: &Path, app_name: &str, force: bool) -> Result<()> {
    let template = standard_template_dir();
    if !template.is_dir() {
        bail!(
            "Standard template not found at {}.\n\
             Reinstall the CLI: cargo install omnisan --force",
            template.display()
        );
    }
    assert_not_template_destination(dest, &template)?;

    let package_name = dest
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("omium-standard-app")
        .to_string();
    let lib_name = package_name.replace('-', "_");

    copy_tree(&template, dest, force, COPY_SKIP_DIRS)?;
    super::template_manifest::activate_manifest_templates(dest)?;
    patch_crate_paths(dest)?;
    patch_identifiers(dest, app_name, &package_name, &lib_name)?;
    super::template_design::write_vite_app_css(dest)?;
    write_app_omnisan_wrapper(dest)?;
    write_env_example(dest, true)?;
    super::standalone_manifest::ensure_standalone_app_workspace(&dest.join("Cargo.toml"))?;
    Ok(())
}

fn standard_template_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates/standard-app")
}

fn assert_not_template_destination(dest: &Path, template: &Path) -> Result<()> {
    let template = template.canonicalize().context("standard template path")?;
    let dest_canon = dest.canonicalize().unwrap_or_else(|_| dest.to_path_buf());
    if dest_canon == template {
        bail!(
            "refusing to generate into the standard template directory itself ({}).",
            template.display()
        );
    }
    Ok(())
}

fn patch_crate_paths(root: &Path) -> Result<()> {
    use crate::generator::crate_deps;
    use crate::generator::standalone_manifest;

    let mode = crate_deps::dependency_mode_for_app(root)?;
    if matches!(mode, crate_deps::DependencyMode::Monorepo { .. }) {
        use crate::generator::framework_paths;
        const TEMPLATE_APP_CRATES: &str = "../../crates";
        const TEMPLATE_MIGRATION_CRATES: &str = "../../../crates";
        let app_crates = framework_paths::detect_crate_path(root)?;
        replace_in_file(&root.join("Cargo.toml"), TEMPLATE_APP_CRATES, &app_crates)?;
        let migration_manifest = root.join("migration/Cargo.toml");
        if migration_manifest.is_file() {
            let migration_crates = framework_paths::detect_crate_path(&root.join("migration"))?;
            replace_in_file(
                &migration_manifest,
                TEMPLATE_MIGRATION_CRATES,
                &migration_crates,
            )?;
        }
        return Ok(());
    }

    crate_deps::rewrite_template_manifest(&root.join("Cargo.toml"), &mode, root)?;
    let migration_manifest = root.join("migration/Cargo.toml");
    if migration_manifest.is_file() {
        crate_deps::rewrite_template_manifest(&migration_manifest, &mode, root)?;
    }
    if matches!(mode, crate_deps::DependencyMode::Registry) {
        standalone_manifest::finalize_standalone_manifest(&root.join("Cargo.toml"))?;
        if migration_manifest.is_file() {
            standalone_manifest::finalize_standalone_manifest(&migration_manifest)?;
        }
    }
    Ok(())
}

fn patch_identifiers(
    root: &Path,
    app_name: &str,
    package_name: &str,
    lib_name: &str,
) -> Result<()> {
    let migration_package = format!("{package_name}-migration");
    let migration_lib = format!("{lib_name}_migration");

    replace_in_file(&root.join("Cargo.toml"), TEMPLATE_PACKAGE, package_name)?;
    replace_in_file(&root.join("Cargo.toml"), TEMPLATE_LIB, lib_name)?;
    replace_in_file(&root.join("src/main.rs"), TEMPLATE_LIB, lib_name)?;
    replace_in_file(
        &root.join("src/bin/route_list.rs"),
        TEMPLATE_LIB,
        lib_name,
    )?;

    replace_in_file(
        &root.join("migration/Cargo.toml"),
        TEMPLATE_MIGRATION_PACKAGE,
        &migration_package,
    )?;
    replace_in_file(
        &root.join("migration/Cargo.toml"),
        TEMPLATE_MIGRATION_LIB,
        &migration_lib,
    )?;
    replace_in_file(
        &root.join("migration/src/main.rs"),
        TEMPLATE_MIGRATION_LIB,
        &migration_lib,
    )?;
    replace_in_file(
        &root.join("Cargo.toml"),
        TEMPLATE_MIGRATION_PACKAGE,
        &migration_package,
    )?;
    replace_in_file(
        &root.join("Cargo.toml"),
        TEMPLATE_MIGRATION_LIB,
        &migration_lib,
    )?;

    replace_in_file(
        &root.join("config/app.toml"),
        "name = \"Omium\"",
        &format!("name = \"{app_name}\""),
    )?;

    let readme = format!(
        "# {app_name}\n\n\
         Standard (Askama) Omium application — same folder layout as Vue/React/API modes.\n\n\
         ```bash\n\
         ./omnisan dev\n\
         open http://127.0.0.1:8080/\n\
         ```\n"
    );
    fs::write(root.join("README.md"), readme)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn copies_standard_template_with_unified_tree() {
        let template = standard_template_dir();
        if !template.is_dir() {
            return;
        }
        let temp = tempdir().expect("tempdir");
        std::fs::create_dir_all(temp.path().join("crates/omium-core")).expect("core");
        let dest = temp.path().join("demo-standard");
        generate_standard_app(&dest, "Acme", true).expect("generate");
        assert!(dest.join("src/routes/mod.rs").is_file());
        assert!(dest.join("src/controllers/welcome.rs").is_file());
        assert!(dest.join("src/models/mod.rs").is_file());
        assert!(dest.join("resources/views/welcome.html").is_file());
    }
}
