//! `omnisan new --frontend vue` — copy the reference Inertia Vue app template.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use super::omnisan_script::write_app_omnisan_wrapper;
use crate::generator::new::write_env_example;

use crate::generator::template_copy::{COPY_SKIP_DIRS, copy_tree, replace_in_file};

/// Copies `examples/inertia-vue-app` into `dest` and renames crate identifiers.
pub fn generate_vue_app(dest: &Path, app_name: &str, force: bool) -> Result<()> {
    let template = vue_template_dir();
    if !template.is_dir() {
        bail!(
            "Vue template not found at {}.\n\
             Reinstall the CLI so generator templates are included:\n\
             cargo install omnisan --force",
            template.display()
        );
    }
    assert_not_vue_template_destination(dest)?;

    let package_name = dest
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("omium-vue-app")
        .to_string();
    let lib_name = package_name.replace('-', "_");

    copy_tree(&template, dest, force, COPY_SKIP_DIRS)?;
    super::template_manifest::activate_manifest_templates(dest)?;
    patch_vue_crate_paths(dest)?;
    patch_vue_identifiers(dest, app_name, &package_name, &lib_name)?;
    super::template_design::write_vite_app_css(dest)?;
    write_app_omnisan_wrapper(dest)?;
    write_env_example(dest, true)?;
    super::standalone_manifest::ensure_standalone_app_workspace(&dest.join("Cargo.toml"))?;
    Ok(())
}

pub fn print_vue_next_steps(path: &Path, package_name: &str) {
    println!("Generated Vue + Inertia application at {}", path.display());
    println!("Next steps:");
    println!("  cd {}", path.display());
    println!("  ./omnisan install");
    println!("  ./omnisan migrate");
    println!("  ./omnisan dev");
    println!("  open http://127.0.0.1:3002/");
    println!();
    println!(
        "If this app lives under the monorepo workspace, add \"{}\" and \"{}/migration\" to the root Cargo.toml [workspace.members].",
        path.display(),
        path.display()
    );
    println!("Or run from the app directory with: ./omnisan dev  (no global install; uses Cargo)");
    let _ = package_name;
}

fn vue_template_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates/vue-inertia-app")
}

/// Rewrites framework `path` / registry deps in generated manifests.
fn patch_vue_crate_paths(root: &Path) -> Result<()> {
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

fn assert_not_vue_template_destination(dest: &Path) -> Result<()> {
    let template = vue_template_dir().canonicalize().context(
        "Vue template directory missing — restore crates/omium-cli/templates/vue-inertia-app",
    )?;
    let dest_canon = dest.canonicalize().unwrap_or_else(|_| dest.to_path_buf());
    if dest_canon == template {
        bail!(
            "refusing to generate into the Vue template directory itself ({}).\n\
             Choose another folder, e.g. examples/my-shop",
            template.display()
        );
    }
    Ok(())
}

fn patch_vue_identifiers(
    root: &Path,
    app_name: &str,
    package_name: &str,
    lib_name: &str,
) -> Result<()> {
    let migration_package = format!("{package_name}-migration");
    let migration_lib = format!("{lib_name}_migration");

    replace_in_file(&root.join("Cargo.toml"), "inertia-vue-app", package_name)?;
    replace_in_file(&root.join("Cargo.toml"), "inertia_vue_app", lib_name)?;

    replace_in_file(&root.join("src/main.rs"), "inertia_vue_app", lib_name)?;
    replace_in_file(
        &root.join("src/bin/route_list.rs"),
        "inertia_vue_app",
        lib_name,
    )?;
    replace_in_file(
        &root.join("tests/inertia_vue.rs"),
        "inertia_vue_app",
        lib_name,
    )?;

    replace_in_file(
        &root.join("migration/Cargo.toml"),
        "inertia-vue-app-migration",
        &migration_package,
    )?;
    replace_in_file(
        &root.join("migration/Cargo.toml"),
        "inertia_vue_app_migration",
        &migration_lib,
    )?;

    replace_in_file(
        &root.join("migration/src/main.rs"),
        "inertia_vue_app_migration",
        &migration_lib,
    )?;

    replace_in_file(
        &root.join("Cargo.toml"),
        "inertia-vue-app-migration",
        &migration_package,
    )?;
    replace_in_file(
        &root.join("Cargo.toml"),
        "inertia_vue_app_migration",
        &migration_lib,
    )?;

    replace_in_file(
        &root.join("config/app.toml"),
        "name = \"Omium\"",
        &format!("name = \"{app_name}\""),
    )?;

    let readme = format!(
        "# {app_name}\n\n\
         Vue + Inertia application generated from Omium.\n\n\
         ```bash\n\
         ./omnisan install\n\
         export APP_DATABASE__ENABLED=true APP_DATABASE__PASSWORD=omium\n\
         ./omnisan migrate\n\
         ./omnisan dev\n\
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
    fn copies_vue_template_with_new_crate_name() {
        let template = vue_template_dir();
        if !template.is_dir() {
            return;
        }
        let temp = tempdir().expect("tempdir");
        std::fs::create_dir_all(temp.path().join("crates/omium-core")).expect("core");
        let dest = temp.path().join("examples/demo-vue-app");
        generate_vue_app(&dest, "Acme", true).expect("generate");
        let cargo = fs::read_to_string(dest.join("Cargo.toml")).expect("cargo");
        assert!(cargo.contains("name = \"demo-vue-app\""));
        assert!(cargo.contains("name = \"demo_vue_app\""));
        assert!(cargo.contains("../../crates/omium-core"));
        assert!(dest.join("resources/js/Pages/Dashboard.vue").is_file());
    }
}
