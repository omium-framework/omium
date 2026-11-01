use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailError {
    #[error("mail is misconfigured: {0}")]
    Config(String),
    #[error("failed to send mail: {0}")]
    Send(String),
    #[error("failed to read mail config: {0}")]
    Io(#[from] std::io::Error),
}
