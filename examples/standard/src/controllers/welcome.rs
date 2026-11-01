//! Welcome page (`GET /`).

use axum::extract::State;
use axum::response::Html;
use axum::Extension;
use omium_auth::SessionHandle;
use omium_core::presentation::RequestPresentation;

use crate::app::{RuntimeError, RuntimeState};
use crate::http::render_html;
use crate::views::WelcomeTemplate;

/// `GET /`
pub async fn show(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(session): Extension<SessionHandle>,
) -> Result<Html<String>, RuntimeError> {
    let authenticated = session.read().await.user_id().is_some();
    let app_name = state.app.config().app.name.clone();
    render_html(WelcomeTemplate {
        app_name: app_name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        authenticated,
        greeting: format!("Welcome to {app_name}"),
        hint: String::from(
            "This page is rendered on the server with Askama — no JavaScript bundle required.",
        ),
        mode_label: String::from("Standard HTML"),
    })
}
