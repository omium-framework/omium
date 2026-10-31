//! `omnisan new` command.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Args, ValueEnum};

use crate::generator::framework_paths;
use crate::generator::new_prompts;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FrontendMode {
    Api,
    Standard,
    Vue,
    React,
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// Directory for the generated application (relative to current directory)
    pub path: PathBuf,

    /// Application display name (prompted when omitted in an interactive terminal)
    #[arg(long)]
    pub app_name: Option<String>,

    /// Frontend mode (prompted when omitted in an interactive terminal)
    #[arg(long, value_enum)]
    pub frontend: Option<FrontendMode>,

    /// Overwrite existing files without confirmation
    #[arg(long)]
    pub yes: bool,

    /// Non-interactive: use defaults (name from path, frontend vue)
    #[arg(long)]
    pub no_interaction: bool,
}

/// Derives a human-readable app name from the project directory name.
#[must_use]
pub fn default_app_name_from_path(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("app")
        .split('-')
        .filter(|segment| !segment.is_empty())
        .map(title_case_segment)
        .collect::<Vec<_>>()
        .join(" ")
}

fn title_case_segment(segment: &str) -> String {
    let mut chars = segment.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Generates a new application skeleton.
pub fn new_app(args: &NewArgs) -> Result<()> {
    let path = framework_paths::resolve_project_path(&args.path)?;
    let app_name =
        new_prompts::resolve_app_name(&path, args.app_name.as_deref(), args.no_interaction)?;
    let frontend = new_prompts::resolve_frontend(args.frontend, args.no_interaction)?;

    if path.exists() {
        if !path.is_dir() {
            bail!("target path exists and is not a directory");
        }
        if fs::read_dir(&path)?.next().is_some() && !args.yes {
            bail!(
                "target directory is not empty: {} (use --yes to continue anyway)",
                path.display()
            );
        }
    } else {
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create directory {}", path.display()))?;
    }

    match frontend {
        FrontendMode::Api => {
            crate::generator::api::generate_api_app(&path, &app_name, args.yes)?;
            print_api_next_steps(&path);
        }
        FrontendMode::Standard => {
            crate::generator::standard::generate_standard_app(&path, &app_name, args.yes)?;
            print_standard_next_steps(&path);
        }
        FrontendMode::Vue => {
            let package_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("omium-vue-app")
                .to_string();
            crate::generator::vue::generate_vue_app(&path, &app_name, args.yes)?;
            crate::generator::vue::print_vue_next_steps(&path, &package_name);
        }
        FrontendMode::React => {
            let package_name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("omium-react-app")
                .to_string();
            crate::generator::react::generate_react_app(&path, &app_name, args.yes)?;
            crate::generator::react::print_react_next_steps(&path, &package_name);
        }
    }

    Ok(())
}

fn print_api_next_steps(path: &Path) {
    println!("Generated API application at {}", path.display());
    println!("Next steps:");
    println!("  cd {}", path.display());
    println!("  ./omnisan dev");
}

fn print_standard_next_steps(path: &Path) {
    println!(
        "Generated standard (Askama) application at {}",
        path.display()
    );
    println!("Next steps:");
    println!("  cd {}", path.display());
    println!("  ./omnisan dev");
    println!("  open http://127.0.0.1:8080/");
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

fn env_example_dotenv() -> &'static str {
    r#"# Local development only — copy to .env (./omnisan install does this once).
# Password must not live in config/*.toml files.
APP_DATABASE__PASSWORD=omium
"#
}

pub(crate) fn write_env_example(base: &Path, force: bool) -> Result<()> {
    write_file(base, ".env.example", env_example_dotenv(), force)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_api_app_tree() {
        let temp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(temp.path().join("crates/omium-core")).expect("crates");
        new_app(&NewArgs {
            path: temp.path().join("demo-api"),
            app_name: Some(String::from("Omium")),
            frontend: Some(FrontendMode::Api),
            yes: true,
            no_interaction: true,
        })
        .expect("generate");
        assert!(temp.path().join("demo-api/src/routes/mod.rs").is_file());
        assert!(
            temp.path()
                .join("demo-api/src/controllers/mod.rs")
                .is_file()
        );
        assert!(temp.path().join("demo-api/config/app.toml").is_file());
    }

    #[test]
    fn generates_standard_app_tree() {
        let temp = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(temp.path().join("crates/omium-core")).expect("core");
        new_app(&NewArgs {
            path: temp.path().join("demo-standard"),
            app_name: Some(String::from("Omium")),
            frontend: Some(FrontendMode::Standard),
            yes: true,
            no_interaction: true,
        })
        .expect("generate");
        assert!(
            temp.path()
                .join("demo-standard/resources/views/welcome.html")
                .is_file()
        );
        assert!(
            temp.path()
                .join("demo-standard/src/controllers/welcome.rs")
                .is_file()
        );
        assert!(
            temp.path()
                .join("demo-standard/src/models/mod.rs")
                .is_file()
        );
    }
}
