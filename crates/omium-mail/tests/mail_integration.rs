use std::sync::Arc;

use omium_mail::{LogMailer, MailSettings, OutboundMail, SharedMailer};

#[tokio::test]
async fn log_mailer_records_outbound_message() {
    let log = Arc::new(LogMailer::default());
    let mailer: SharedMailer = log.clone();
    mailer
        .send(OutboundMail::new(
            "admin@omium.local",
            "Reset password",
            "Click the link",
        ))
        .await
        .expect("send");
    assert_eq!(log.snapshot().len(), 1);
}

#[test]
fn default_mail_settings_match_mailpit() {
    let settings = MailSettings::default();
    assert_eq!(settings.host, "127.0.0.1");
    assert_eq!(settings.port, 1025);
}
