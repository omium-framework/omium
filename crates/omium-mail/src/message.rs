/// Simple plaintext email.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutboundMail {
    pub to: String,
    pub subject: String,
    pub body_text: String,
}

impl OutboundMail {
    #[must_use]
    pub fn new(to: impl Into<String>, subject: impl Into<String>, body_text: impl Into<String>) -> Self {
        Self {
            to: to.into(),
            subject: subject.into(),
            body_text: body_text.into(),
        }
    }
}
