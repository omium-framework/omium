//! SMTP and log mail drivers (Mailpit-friendly for local dev).

mod error;
mod mailer;
mod message;
mod settings;

pub use error::MailError;
pub use mailer::{LogMailer, Mailer, SharedMailer, SmtpMailer, build_mailer, log_mailer, smtp_mailer};
pub use message::OutboundMail;
pub use settings::{MailSettings, load_mail_settings};
