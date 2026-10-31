//! Load configuration with precedence: defaults < TOML files < `APP_` environment.

use std::path::Path;

use toml::Value;

use crate::error::ConfigError;

use super::settings::AppConfig;

/// Environment variable prefix for configuration overrides (see `docs/ARCHITECTURE.md`).
pub const ENV_PREFIX: &str = "APP_";

const CONFIG_FILES: &[&str] = &["app.toml", "logging.toml", "server.toml", "database.toml"];

/// Loads configuration from `config_dir`, applying defaults, file merges, and environment overrides.
///
/// # Errors
///
/// Returns [`ConfigError`] when the directory is missing, files are invalid, secrets appear in files, or validation fails.
pub fn load_config(config_dir: impl AsRef<Path>) -> Result<AppConfig, ConfigError> {
    let config_dir = config_dir.as_ref();
    load_dotenv_from_app_root(config_dir);
    load_from_dir(config_dir, std::env::vars())
}

#[cfg(test)]
pub(crate) fn load_config_with_vars(
    config_dir: impl AsRef<Path>,
    vars: impl IntoIterator<Item = (String, String)>,
) -> Result<AppConfig, ConfigError> {
    load_from_dir(config_dir, vars)
}

fn load_from_dir(
    config_dir: impl AsRef<Path>,
    vars: impl IntoIterator<Item = (String, String)>,
) -> Result<AppConfig, ConfigError> {
    let config_dir = config_dir.as_ref();
    if !config_dir.is_dir() {
        return Err(ConfigError::ConfigDirMissing {
            path: config_dir.to_path_buf(),
        });
    }

    let default_toml = toml::to_string(&AppConfig::default()).map_err(ConfigError::Serialize)?;
    let mut merged: Value = toml::from_str(&default_toml).map_err(ConfigError::Deserialize)?;

    for file_name in CONFIG_FILES {
        let path = config_dir.join(file_name);
        if !path.is_file() {
            continue;
        }
        let contents = std::fs::read_to_string(&path).map_err(|source| ConfigError::ReadFile {
            path: path.clone(),
            source,
        })?;
        let file_value: Value =
            toml::from_str(&contents).map_err(|source| ConfigError::ParseFile {
                path: path.clone(),
                source,
            })?;
        merge_values(&mut merged, file_value);
    }

    let from_files: AppConfig = merged
        .clone()
        .try_into()
        .map_err(ConfigError::Deserialize)?;

    if from_files.database.password.is_some() {
        return Err(ConfigError::PasswordInConfigFile);
    }

    let mut config: AppConfig = merged.try_into().map_err(ConfigError::Deserialize)?;
    apply_env_overrides_from(&mut config, vars)?;
    apply_database_auto_enable(&mut config);
    validate(&config)?;
    Ok(config)
}

/// Loads `.env` from the application root (`config/` parent) when present.
///
/// Existing process environment variables are not overridden (same as Laravel `.env` after export).
fn load_dotenv_from_app_root(config_dir: &Path) {
    let Some(app_root) = config_dir.parent() else {
        return;
    };
    let env_path = app_root.join(".env");
    if env_path.is_file() {
        let _ = dotenvy::from_path(&env_path);
    }
}

/// Enables `MySQL` when a password is configured via environment (including `.env`).
fn apply_database_auto_enable(config: &mut AppConfig) {
    if std::env::var(format!("{ENV_PREFIX}DATABASE__ENABLED"))
        .is_ok_and(|value| value.eq_ignore_ascii_case("false"))
    {
        return;
    }

    let has_password = config
        .database
        .password
        .as_deref()
        .is_some_and(|password| !password.trim().is_empty());

    if has_password {
        config.database.enabled = true;
    }
}

fn merge_values(base: &mut Value, overlay: Value) {
    match (base, overlay) {
        (Value::Table(base_table), Value::Table(overlay_table)) => {
            for (key, overlay_value) in overlay_table {
                match base_table.get_mut(&key) {
                    Some(existing) => merge_values(existing, overlay_value),
                    None => {
                        base_table.insert(key, overlay_value);
                    }
                }
            }
        }
        (base_slot, overlay) => {
            *base_slot = overlay;
        }
    }
}

fn apply_env_overrides_from(
    config: &mut AppConfig,
    vars: impl IntoIterator<Item = (String, String)>,
) -> Result<(), ConfigError> {
    for (key, value) in vars {
        if !key.starts_with(ENV_PREFIX) {
            continue;
        }
        let remainder = key
            .strip_prefix(ENV_PREFIX)
            .ok_or_else(|| ConfigError::EnvOverride {
                key: key.clone(),
                message: String::from("invalid prefix"),
            })?;

        if let Some(stripped) = remainder.strip_prefix("APP__") {
            apply_nested_override(config, stripped, &value, &key)?;
            continue;
        }

        match remainder {
            "NAME" => config.app.name = value,
            "ENV" | "ENVIRONMENT" => config.app.environment = value,
            "DEBUG" => {
                config.app.debug =
                    parse_bool(&value).map_err(|message| ConfigError::EnvOverride {
                        key: key.clone(),
                        message,
                    })?;
            }
            "LOG_LEVEL" | "LOGGING__LEVEL" => config.logging.level = value.to_ascii_lowercase(),
            "DATABASE__PASSWORD" => config.database.password = Some(value),
            "DATABASE__HOST" => config.database.host = value,
            "DATABASE__PORT" => {
                config.database.port = value.parse().map_err(|_| ConfigError::EnvOverride {
                    key: key.clone(),
                    message: String::from("port must be a valid u16"),
                })?;
            }
            "DATABASE__NAME" => config.database.name = value,
            "DATABASE__USERNAME" => config.database.username = value,
            "DATABASE__ENABLED" => {
                config.database.enabled =
                    parse_bool(&value).map_err(|message| ConfigError::EnvOverride {
                        key: key.clone(),
                        message,
                    })?;
            }
            "SERVER__HOST" => config.server.host = value,
            "SERVER__PORT" => {
                config.server.port = value.parse().map_err(|_| ConfigError::EnvOverride {
                    key: key.clone(),
                    message: String::from("port must be a valid u16"),
                })?;
            }
            "SERVER__REQUEST_TIMEOUT_SECS" => {
                config.server.request_timeout_secs =
                    value.parse().map_err(|_| ConfigError::EnvOverride {
                        key: key.clone(),
                        message: String::from("request_timeout_secs must be a valid u64"),
                    })?;
            }
            "SERVER__BODY_LIMIT_BYTES" => {
                config.server.body_limit_bytes =
                    value.parse().map_err(|_| ConfigError::EnvOverride {
                        key: key.clone(),
                        message: String::from("body_limit_bytes must be a valid usize"),
                    })?;
            }
            other if other.contains("__") => {
                apply_nested_override(config, other, &value, &key)?;
            }
            other => {
                return Err(ConfigError::EnvOverride {
                    key: key.clone(),
                    message: format!("unsupported configuration key `{other}`"),
                });
            }
        }
    }
    Ok(())
}

fn apply_nested_override(
    config: &mut AppConfig,
    path: &str,
    value: &str,
    env_key: &str,
) -> Result<(), ConfigError> {
    let segments: Vec<&str> = path.split("__").collect();
    match segments.as_slice() {
        ["APP", "NAME"] => config.app.name = value.to_string(),
        ["APP", "ENV" | "ENVIRONMENT"] => config.app.environment = value.to_string(),
        ["APP", "DEBUG"] => {
            config.app.debug = parse_bool(value).map_err(|message| ConfigError::EnvOverride {
                key: env_key.to_string(),
                message,
            })?;
        }
        ["LOGGING", "LEVEL"] => config.logging.level = value.to_ascii_lowercase(),
        ["DATABASE", "PASSWORD"] => config.database.password = Some(value.to_string()),
        ["DATABASE", "HOST"] => config.database.host = value.to_string(),
        ["DATABASE", "PORT"] => {
            config.database.port = value.parse().map_err(|_| ConfigError::EnvOverride {
                key: env_key.to_string(),
                message: String::from("port must be a valid u16"),
            })?;
        }
        ["DATABASE", "NAME"] => config.database.name = value.to_string(),
        ["DATABASE", "USERNAME"] => config.database.username = value.to_string(),
        ["DATABASE", "ENABLED"] => {
            config.database.enabled =
                parse_bool(value).map_err(|message| ConfigError::EnvOverride {
                    key: env_key.to_string(),
                    message,
                })?;
        }
        ["SERVER", "HOST"] => config.server.host = value.to_string(),
        ["SERVER", "PORT"] => {
            config.server.port = value.parse().map_err(|_| ConfigError::EnvOverride {
                key: env_key.to_string(),
                message: String::from("port must be a valid u16"),
            })?;
        }
        ["SERVER", "REQUEST_TIMEOUT_SECS"] => {
            config.server.request_timeout_secs =
                value.parse().map_err(|_| ConfigError::EnvOverride {
                    key: env_key.to_string(),
                    message: String::from("request_timeout_secs must be a valid u64"),
                })?;
        }
        ["SERVER", "BODY_LIMIT_BYTES"] => {
            config.server.body_limit_bytes =
                value.parse().map_err(|_| ConfigError::EnvOverride {
                    key: env_key.to_string(),
                    message: String::from("body_limit_bytes must be a valid usize"),
                })?;
        }
        _ => {
            return Err(ConfigError::EnvOverride {
                key: env_key.to_string(),
                message: format!("unsupported nested path `{path}`"),
            });
        }
    }
    Ok(())
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Ok(true),
        "0" | "false" | "no" | "off" => Ok(false),
        _ => Err(String::from("expected true/false, 1/0, yes/no, or on/off")),
    }
}

fn validate(config: &AppConfig) -> Result<(), ConfigError> {
    if config.app.name.trim().is_empty() {
        return Err(ConfigError::EmptyAppName);
    }

    if !is_valid_log_level(&config.logging.level) {
        return Err(ConfigError::InvalidLogLevel {
            value: config.logging.level.clone(),
        });
    }

    if config.database.enabled && config.database.password.as_deref().unwrap_or("").is_empty() {
        return Err(ConfigError::MissingDatabasePassword);
    }

    if config.server.port == 0 {
        return Err(ConfigError::InvalidServerPort);
    }

    if config.server.request_timeout_secs == 0 {
        return Err(ConfigError::InvalidRequestTimeout);
    }

    if config.server.body_limit_bytes == 0 {
        return Err(ConfigError::InvalidBodyLimit);
    }

    Ok(())
}

fn is_valid_log_level(level: &str) -> bool {
    matches!(
        level.to_ascii_lowercase().as_str(),
        "trace" | "debug" | "info" | "warn" | "error"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_config(dir: &Path, name: &str, contents: &str) {
        std::fs::write(dir.join(name), contents).expect("write config");
    }

    #[test]
    fn precedence_defaults_files_then_env() {
        let temp = tempfile::tempdir().expect("tempdir");
        write_config(
            temp.path(),
            "app.toml",
            r#"
[app]
name = "From File"
debug = true
"#,
        );
        write_config(
            temp.path(),
            "logging.toml",
            r#"
[logging]
level = "warn"
"#,
        );

        let config = load_config_with_vars(
            temp.path(),
            [
                ("APP_NAME".to_string(), "From Env".to_string()),
                ("APP_DEBUG".to_string(), "false".to_string()),
            ],
        )
        .expect("load config");
        assert_eq!(config.app.name, "From Env");
        assert!(!config.app.debug);
        assert_eq!(config.logging.level, "warn");
    }

    #[test]
    fn nested_env_override_supported() {
        let temp = tempfile::tempdir().expect("tempdir");
        write_config(
            temp.path(),
            "app.toml",
            r#"
[app]
name = "Nested Test"
"#,
        );

        let config = load_config_with_vars(
            temp.path(),
            [("APP_LOGGING__LEVEL".to_string(), "debug".to_string())],
        )
        .expect("load config");
        assert_eq!(config.logging.level, "debug");
    }

    #[test]
    fn invalid_config_returns_actionable_error() {
        let temp = tempfile::tempdir().expect("tempdir");
        write_config(
            temp.path(),
            "app.toml",
            r#"
[app]
name = ""
"#,
        );

        let err = load_config(temp.path()).expect_err("expected error");
        assert!(matches!(err, ConfigError::EmptyAppName));
    }

    #[test]
    fn password_in_config_file_is_rejected() {
        let temp = tempfile::tempdir().expect("tempdir");
        write_config(
            temp.path(),
            "app.toml",
            r#"
[app]
name = "Secret Test"
"#,
        );
        write_config(
            temp.path(),
            "database.toml",
            r#"
[database]
enabled = true
password = "not-allowed"
"#,
        );

        let err = load_config(temp.path()).expect_err("expected error");
        assert!(matches!(err, ConfigError::PasswordInConfigFile));
    }

    #[test]
    fn dotenv_enables_database_when_password_present() {
        let temp = tempfile::tempdir().expect("tempdir");
        let app_root = temp.path().join("app");
        let config_dir = app_root.join("config");
        std::fs::create_dir_all(&config_dir).expect("config dir");
        write_config(
            &config_dir,
            "app.toml",
            r#"
[app]
name = "Dotenv Test"
"#,
        );
        write_config(
            &config_dir,
            "database.toml",
            r"
[database]
enabled = false
",
        );
        std::fs::write(app_root.join(".env"), "APP_DATABASE__PASSWORD=omium\n").expect("write env");

        let config = load_config(&config_dir).expect("load config");
        assert!(config.database.enabled);
        assert_eq!(config.database.password.as_deref(), Some("omium"));
    }

    #[test]
    fn debug_output_redacts_database_password() {
        let mut config = AppConfig::default();
        config.database.enabled = true;
        config.database.password = Some(String::from("super-secret"));

        let debug = format!("{config:?}");
        assert!(!debug.contains("super-secret"));
        assert!(debug.contains("REDACTED"));
    }
}
