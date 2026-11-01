//! Demo registration (Askama).

use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Extension;
use axum::Form;
use omium_auth::{verify_token, SessionHandle};
use omium_core::presentation::RequestPresentation;
use omium_mail::OutboundMail;

use crate::app::{DemoUserStore, RuntimeError, RuntimeState, effective_auth};
use crate::forms::RegisterForm;
use crate::http::render_html;
use crate::views::RegisterTemplate;

/// `GET /register`
pub async fn show_register(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
) -> Result<Response, RuntimeError> {
    let html = render_html(RegisterTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        error: None,
    })?;
    Ok(html.into_response())
}

/// `POST /register`
pub async fn register(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(session): Extension<SessionHandle>,
    Form(form): Form<RegisterForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return Ok(axum::response::Html(String::from("CSRF token mismatch")).into_response());
        }
    }

    let auth = effective_auth(&state);
    let email = form.email.trim();
    let password = form.password.trim();

    let error = if password.len() < 8 {
        Some(String::from("Password must be at least 8 characters."))
    } else if form.password != form.password_confirmation {
        Some(String::from("Passwords do not match."))
    } else {
        match DemoUserStore::register(email, password, &auth.demo.email) {
            Ok(_) => None,
            Err(_) => Some(String::from("That email is already registered.")),
        }
    };

    if let Some(message) = error {
        let html = render_html(RegisterTemplate {
            app_name: state.app.config().app.name.clone(),
            nav: presentation.nav,
            flash: presentation.flash,
            csrf_token: presentation.csrf_token,
            error: Some(message),
        })?;
        return Ok(html.into_response());
    }

    let user = DemoUserStore::find_by_email(form.email.trim()).expect("registered user");
    let verify_token = state.email_verification.issue(&user.email)?;
    let verify_url = format!(
        "http://127.0.0.1:{}/email/verify/{verify_token}",
        state.app.config().server.port
    );
    state
        .mailer
        .send(OutboundMail::new(
            &user.email,
            "Verify your email",
            format!(
                "Hi {},\n\nConfirm your email:\n\n{verify_url}",
                form.name.trim()
            ),
        ))
        .await?;

    Ok(Redirect::to("/login").into_response())
}
