//! `config/mail.toml` loader.

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::MailError;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct MailSettings {
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub from_address: String,
    pub from_name: String,
}

impl Default for MailSettings {
    fn default() -> Self {
        Self {
            driver: String::from("smtp"),
            host: String::from("127.0.0.1"),
            port: 1025,
            from_address: String::from("noreply@omium.local"),
            from_name: String::from("Omium"),
        }
    }
}

/// Loads `mail.toml` from the configuration directory (missing or invalid → defaults).
///
/// # Errors
///
/// Returns [`MailError::Io`] when the file exists but cannot be read.
pub fn load_mail_settings(config_dir: &Path) -> Result<MailSettings, MailError> {
    let path = config_dir.join("mail.toml");
    if !path.is_file() {
        return Ok(MailSettings::default());
    }
    let contents = fs::read_to_string(path)?;
    toml::from_str(&contents).map_err(|error| MailError::Config(error.to_string()))
}
