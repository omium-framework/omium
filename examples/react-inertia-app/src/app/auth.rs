//! Demo login settings (`config/auth.toml`) — replace with a `users` table in production.

use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthSettings {
    pub demo: DemoLogin,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DemoLogin {
    pub enabled: bool,
    pub email: String,
    pub password: String,
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            demo: DemoLogin {
                enabled: true,
                email: String::from("admin@omium.local"),
                password: String::from("omium-dev"),
            },
        }
    }
}

/// Loads `auth.toml` from the configuration directory (missing or invalid → defaults).
#[must_use]
pub fn load_auth_settings(config_dir: &Path) -> AuthSettings {
    let path = config_dir.join("auth.toml");
    if !path.is_file() {
        return AuthSettings::default();
    }
    let Ok(contents) = fs::read_to_string(path) else {
        return AuthSettings::default();
    };
    toml::from_str(&contents).unwrap_or_default()
}
