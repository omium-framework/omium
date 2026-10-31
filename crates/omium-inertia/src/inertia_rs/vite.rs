//! Vite dev-server and production manifest tags for Inertia root views.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

use serde::Deserialize;

/// Errors while reading a Vite build manifest.
#[derive(Debug, thiserror::Error)]
pub enum ViteAssetError {
    #[error("failed to read vite manifest at {path}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse vite manifest")]
    Parse(#[from] serde_json::Error),

    #[error("entry {entry} not found in vite manifest")]
    EntryNotFound { entry: String },
}

#[derive(Debug, Deserialize)]
struct ManifestEntry {
    file: String,
    css: Option<Vec<String>>,
}

/// Script tags pointing at a running Vite dev server (Laravel `public/hot` equivalent via env).
#[must_use]
pub fn vite_dev_tags(dev_server_origin: &str, entry: &str) -> String {
    let base = dev_server_origin.trim_end_matches('/');
    format!(
        r#"<script type="module" src="{base}/@vite/client"></script>
<script type="module" src="{base}/{entry}"></script>"#
    )
}

/// Like [`vite_dev_tags`], plus `@vitejs/plugin-react` refresh preamble (required when HTML is not Vite's `index.html`).
#[must_use]
pub fn vite_react_dev_tags(dev_server_origin: &str, entry: &str) -> String {
    let base = dev_server_origin.trim_end_matches('/');
    format!(
        r#"<script type="module">
import RefreshRuntime from "{base}/@react-refresh";
RefreshRuntime.injectIntoGlobalHook(window);
window.$RefreshReg$ = () => {{}};
window.$RefreshSig$ = () => (type) => type;
window.__vite_plugin_react_preamble_installed__ = true;
</script>
<script type="module" src="{base}/@vite/client"></script>
<script type="module" src="{base}/{entry}"></script>"#
    )
}

/// `<script>` / `<link>` tags from a Vite `manifest.json` after `npm run build`.
///
/// # Errors
///
/// Returns [`ViteAssetError`] when the manifest is missing, invalid, or lacks the entry key.
pub fn vite_production_tags(manifest_path: &Path, entry: &str) -> Result<String, ViteAssetError> {
    let raw = std::fs::read_to_string(manifest_path).map_err(|source| ViteAssetError::Read {
        path: manifest_path.display().to_string(),
        source,
    })?;
    let manifest: HashMap<String, ManifestEntry> = serde_json::from_str(&raw)?;
    let entry_record = manifest.get(entry).ok_or(ViteAssetError::EntryNotFound {
        entry: entry.to_string(),
    })?;

    let mut tags = String::new();
    if let Some(stylesheets) = &entry_record.css {
        for href in stylesheets {
            let _ = write!(tags, r#"<link rel="stylesheet" href="/build/{href}">"#);
            tags.push('\n');
        }
    }
    let _ = write!(
        tags,
        r#"<script type="module" src="/build/{}"></script>"#,
        entry_record.file
    );
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn react_dev_tags_include_refresh_preamble() {
        let tags = vite_react_dev_tags("http://127.0.0.1:5173", "resources/js/app.tsx");
        assert!(tags.contains("__vite_plugin_react_preamble_installed__"));
        assert!(tags.contains("@react-refresh"));
        assert!(tags.contains("resources/js/app.tsx"));
    }

    #[test]
    fn dev_tags_point_at_vite_entry() {
        let tags = vite_dev_tags("http://127.0.0.1:5173", "resources/js/app.ts");
        assert!(tags.contains("@vite/client"));
        assert!(tags.contains("resources/js/app.ts"));
    }

    #[test]
    fn production_tags_read_manifest() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("manifest.json");
        std::fs::write(
            &path,
            r#"{
              "resources/js/app.ts": {
                "file": "assets/app-abc.js",
                "css": ["assets/app-abc.css"]
              }
            }"#,
        )
        .expect("write");
        let tags = vite_production_tags(&path, "resources/js/app.ts").expect("tags");
        assert!(tags.contains("/build/assets/app-abc.js"));
        assert!(tags.contains("/build/assets/app-abc.css"));
    }
}
