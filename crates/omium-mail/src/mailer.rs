//! Mail transport implementations.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use lettre::message::{Mailbox, Message, SinglePart};
use lettre::{SmtpTransport, Transport};

use crate::{MailError, MailSettings, OutboundMail};

/// Sends outbound mail.
#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send(&self, mail: OutboundMail) -> Result<(), MailError>;
}

pub type SharedMailer = Arc<dyn Mailer>;

#[must_use]
pub fn build_mailer(settings: &MailSettings) -> Result<SharedMailer, MailError> {
    if settings.driver == "log" {
        Ok(log_mailer())
    } else {
        smtp_mailer(settings)
    }
}

#[must_use]
pub fn log_mailer() -> SharedMailer {
    Arc::new(LogMailer::default())
}

#[must_use]
pub fn smtp_mailer(settings: &MailSettings) -> Result<SharedMailer, MailError> {
    Ok(Arc::new(SmtpMailer::new(settings)?))
}

/// Records messages in memory (tests and `driver = "log"`).
#[derive(Default)]
pub struct LogMailer {
    sent: Mutex<Vec<OutboundMail>>,
}

impl LogMailer {
    #[must_use]
    pub fn snapshot(&self) -> Vec<OutboundMail> {
        self.sent.lock().expect("log mailer lock").clone()
    }
}

#[async_trait]
impl Mailer for LogMailer {
    async fn send(&self, mail: OutboundMail) -> Result<(), MailError> {
        self.sent.lock().expect("log mailer lock").push(mail);
        Ok(())
    }
}

pub struct SmtpMailer {
    transport: SmtpTransport,
    from: Mailbox,
}

impl SmtpMailer {
    pub fn new(settings: &MailSettings) -> Result<Self, MailError> {
        if settings.driver != "smtp" {
            return Err(MailError::Config(format!(
                "unsupported mail driver `{}` (use smtp or log)",
                settings.driver
            )));
        }
        let transport = SmtpTransport::builder_dangerous(settings.host.clone())
            .port(settings.port)
            .build();
        let from = format!("{} <{}>", settings.from_name, settings.from_address)
            .parse()
            .map_err(|error| MailError::Config(format!("invalid from address: {error}")))?;
        Ok(Self { transport, from })
    }
}

#[async_trait]
impl Mailer for SmtpMailer {
    async fn send(&self, mail: OutboundMail) -> Result<(), MailError> {
        let to: Mailbox = mail
            .to
            .parse()
            .map_err(|error| MailError::Config(format!("invalid to address: {error}")))?;
        let message = Message::builder()
            .from(self.from.clone())
            .to(to)
            .subject(mail.subject)
            .singlepart(SinglePart::plain(mail.body_text))
            .map_err(|error| MailError::Send(error.to_string()))?;
        let transport = self.transport.clone();
        tokio::task::spawn_blocking(move || transport.send(&message))
            .await
            .map_err(|error| MailError::Send(error.to_string()))?
            .map_err(|error| MailError::Send(error.to_string()))?;
        Ok(())
    }
}
