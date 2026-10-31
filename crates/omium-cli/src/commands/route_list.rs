//! `route:list` — print registered application routes.

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Args;

use super::project;

#[derive(Args, Debug)]
pub struct RouteListArgs {
    /// Application directory (defaults to current directory)
    #[arg(long, default_value = ".")]
    pub path: PathBuf,
}

/// Lists HTTP routes for the application in `path`, or framework defaults when not in an app.
///
/// # Errors
///
/// Returns an error when the path is invalid or Cargo fails.
pub async fn route_list(args: RouteListArgs) -> Result<()> {
    let root = args
        .path
        .canonicalize()
        .with_context(|| format!("application path not found: {}", args.path.display()))?;

    if root.join("src/bin/route_list.rs").is_file() && project::looks_like_app_crate(&root) {
        project::cargo_run_in(&root, &["--bin", "route-list"]).await?;
        return Ok(());
    }

    if project::looks_like_app_crate(&root) {
        bail!(
            "this application has no route-list binary (src/bin/route_list.rs). \
             Regenerate from the Vue template or add the file from examples/inertia-vue-app."
        );
    }

    print_framework_defaults();
    Ok(())
}

fn print_framework_defaults() {
    println!("{:<7}  {}", "Method", "URI");
    println!("{}", "-".repeat(40));
    for (method, path) in omium_http::default_route_manifest() {
        println!("{method:<7}  {path}");
    }
    println!();
    println!("Run `./omnisan route:list` from an application directory for app routes.");
}
