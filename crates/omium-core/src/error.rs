//! Typed errors for core bootstrap and configuration.

use std::path::PathBuf;

use thiserror::Error;

/// Errors while loading or validating application configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("configuration directory does not exist: {}", path.display())]
    ConfigDirMissing { path: PathBuf },

    #[error("failed to read config file {}: {source}", path.display())]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse config file {}: {source}", path.display())]
    ParseFile {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[error("failed to serialize default configuration: {0}")]
    Serialize(#[from] toml::ser::Error),

    #[error("failed to deserialize merged configuration: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("invalid environment override for `{key}`: {message}")]
    EnvOverride { key: String, message: String },

    #[error("app.name must not be empty")]
    EmptyAppName,

    #[error("logging.level must be one of trace, debug, info, warn, error (got `{value}`)")]
    InvalidLogLevel { value: String },

    #[error("database.password must be supplied via environment, not config files")]
    PasswordInConfigFile,

    #[error("database.password is required when database settings are present")]
    MissingDatabasePassword,

    #[error("server.port must be greater than zero")]
    InvalidServerPort,

    #[error("server.request_timeout_secs must be greater than zero")]
    InvalidRequestTimeout,

    #[error("server.body_limit_bytes must be greater than zero")]
    InvalidBodyLimit,
}

/// Errors during application bootstrap.
#[derive(Debug, Error)]
pub enum BootstrapError {
    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("failed to initialize tracing: {0}")]
    TracingInit(String),
}
