//! Mail transport bootstrap (`config/mail.toml`).

use std::path::Path;

use omium_mail::{MailError, MailSettings, SharedMailer, build_mailer, load_mail_settings};

/// Loads mail settings and builds the configured driver (`smtp` or `log`).
///
/// # Errors
///
/// Returns [`MailError`] when configuration or SMTP transport setup fails.
pub fn build_app_mailer(config_dir: &Path) -> Result<(MailSettings, SharedMailer), MailError> {
    let settings = load_mail_settings(config_dir)?;
    let mailer = build_mailer(&settings)?;
    Ok((settings, mailer))
}
