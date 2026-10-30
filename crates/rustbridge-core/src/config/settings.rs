//! Configuration value types.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Root configuration aggregate.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub app: AppSettings,
    pub logging: LoggingSettings,
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("app", &self.app)
            .field("logging", &self.logging)
            .field("server", &self.server)
            .field("database", &self.database.debug_redacted())
            .finish()
    }
}

/// Application metadata and runtime flags.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub name: String,
    pub environment: String,
    pub debug: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            name: String::from("Omium"),
            environment: String::from("local"),
            debug: false,
        }
    }
}

/// Logging configuration for `tracing-subscriber`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggingSettings {
    pub level: String,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: String::from("info"),
        }
    }
}

/// HTTP server bind address and request policy limits.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub request_timeout_secs: u64,
    pub body_limit_bytes: usize,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 8080,
            request_timeout_secs: 30,
            body_limit_bytes: 2 * 1024 * 1024,
        }
    }
}

/// Database connection settings. Password must come from the environment only.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseSettings {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            host: String::from("127.0.0.1"),
            port: 3306,
            name: String::from("app"),
            username: String::from("app"),
            password: None,
        }
    }
}

impl DatabaseSettings {
    pub(crate) fn debug_redacted(&self) -> DatabaseSettingsRedacted<'_> {
        DatabaseSettingsRedacted { inner: self }
    }
}

pub(crate) struct DatabaseSettingsRedacted<'a> {
    inner: &'a DatabaseSettings,
}

impl fmt::Debug for DatabaseSettingsRedacted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseSettings")
            .field("enabled", &self.inner.enabled)
            .field("host", &self.inner.host)
            .field("port", &self.inner.port)
            .field("name", &self.inner.name)
            .field("username", &self.inner.username)
            .field(
                "password",
                &self
                    .inner
                    .password
                    .as_ref()
                    .map_or("<unset>", |_| "<REDACTED>"),
            )
            .finish()
    }
}
