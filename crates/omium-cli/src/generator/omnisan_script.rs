//! Writes the app-local `./omnisan` wrapper into generated projects.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

const APP_WRAPPER: &str = include_str!("../../embed/omnisan-app-wrapper.sh");

/// Creates an executable `./omnisan` script in the application root.
///
/// # Errors
///
/// Returns an error when the file cannot be written or permissions cannot be set.
pub fn write_app_omnisan_wrapper(root: &Path) -> Result<()> {
    let path = root.join("omnisan");
    fs::write(&path, APP_WRAPPER).with_context(|| format!("write {}", path.display()))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)?;
    }
    Ok(())
}
