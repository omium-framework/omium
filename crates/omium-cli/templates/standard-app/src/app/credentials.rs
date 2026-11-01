//! Demo credential checks (config admin + file-registered users).

use super::{apply_demo_password_override, demo_users::DemoUserStore, AuthSettings, RuntimeState};

pub fn effective_auth(state: &RuntimeState) -> AuthSettings {
    let mut auth = state.auth.clone();
    apply_demo_password_override(&mut auth);
    auth
}

/// Returns session `user_id` when email/password match a demo account.
#[must_use]
pub fn verify_credentials(state: &RuntimeState, email: &str, password: &str) -> Option<i64> {
    let auth = effective_auth(state);
    if !auth.demo.enabled {
        return None;
    }
    let email = email.trim();
    if auth.demo.email == email && auth.demo.password == password {
        return Some(1);
    }
    DemoUserStore::find_by_email(email).and_then(|user| {
        (user.password == password).then_some(user.user_id)
    })
}

#[must_use]
pub fn email_is_registered(state: &RuntimeState, email: &str) -> bool {
    let auth = effective_auth(state);
    DemoUserStore::email_registered(email, &auth.demo.email)
}
