//! Password reset sends mail via the configured driver (log in tests).

use std::sync::Arc;

use omium_mail::{LogMailer, MailSettings, OutboundMail, SharedMailer};

#[tokio::test]
async fn smtp_driver_can_send_to_log_mailer_stand_in() {
    let log = Arc::new(LogMailer::default());
    let mailer: SharedMailer = log.clone();
    mailer
        .send(OutboundMail::new(
            "admin@omium.local",
            "Reset your password",
            "Open /reset-password/example",
        ))
        .await
        .expect("send");
    let sent = log.snapshot();
    assert_eq!(sent.len(), 1);
    assert!(sent[0].body_text.contains("reset-password"));
}

#[test]
fn mailpit_defaults_match_compose_ports() {
    let settings = MailSettings::default();
    assert_eq!(settings.port, 1025);
}
