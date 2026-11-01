//! Demo login and API token settings (`config/auth.toml`).

use std::fs;
use std::path::Path;

use omium_auth::demo_token_store;
use omium_http::ApiTokenMiddlewareState;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AuthSettings {
    pub demo: DemoLogin,
    #[serde(default)]
    pub api: ApiTokenSettings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DemoLogin {
    pub enabled: bool,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApiTokenSettings {
    pub enabled: bool,
    #[serde(default = "default_demo_api_token")]
    pub demo_token: String,
    #[serde(default = "default_demo_user_id")]
    pub demo_user_id: u64,
}

fn default_demo_api_token() -> String {
    String::from("omium-dev-api-token")
}

const fn default_demo_user_id() -> u64 {
    1
}

impl Default for AuthSettings {
    fn default() -> Self {
        Self {
            demo: DemoLogin {
                enabled: true,
                email: String::from("admin@omium.local"),
                password: String::from("omium-dev"),
            },
            api: ApiTokenSettings::default(),
        }
    }
}

impl Default for ApiTokenSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            demo_token: default_demo_api_token(),
            demo_user_id: default_demo_user_id(),
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

#[derive(Deserialize)]
struct DemoPasswordOverride {
    password: String,
}

/// Applies optional `storage/framework/demo_credentials.toml` (password reset demo).
pub fn apply_demo_password_override(settings: &mut AuthSettings) {
    let path = Path::new("storage/framework/demo_credentials.toml");
    if !path.is_file() {
        return;
    }
    let Ok(contents) = fs::read_to_string(path) else {
        return;
    };
    if let Ok(override_file) = toml::from_str::<DemoPasswordOverride>(&contents) {
        settings.demo.password = override_file.password;
    }
}

/// Loads auth settings and demo password override.
#[must_use]
pub fn load_auth_settings_with_overrides(config_dir: &Path) -> AuthSettings {
    let mut settings = load_auth_settings(config_dir);
    apply_demo_password_override(&mut settings);
    settings
}

/// Builds API Bearer middleware state from loaded auth config.
#[must_use]
pub fn build_api_token_middleware(auth: &AuthSettings) -> ApiTokenMiddlewareState {
    if !auth.api.enabled {
        return ApiTokenMiddlewareState::disabled();
    }
    ApiTokenMiddlewareState::new(
        true,
        demo_token_store(&auth.api.demo_token, auth.api.demo_user_id),
    )
}
