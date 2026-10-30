//! Typed configuration loaded from TOML files and `APP_` environment overrides.

mod load;
mod settings;

pub use load::{ENV_PREFIX, load_config};
pub use settings::{AppConfig, AppSettings, DatabaseSettings, LoggingSettings, ServerSettings};
