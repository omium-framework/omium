//! Demo registration (file store + verification email).

use axum::extract::State;
use axum::response::Response;
use axum::Extension;
use omium_auth::{verify_token, SessionHandle};
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::{Inertia, InertiaForm, InertiaRequest};
use omium_mail::OutboundMail;
use serde_json::json;

use crate::app::{DemoUserStore, RuntimeError, RuntimeState, effective_auth};
use crate::forms::RegisterForm;
use crate::http::render_inertia;
use crate::middleware::Authenticated;

/// `GET /register`
pub async fn show_register(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "Register",
        "Create account",
        |props| props.value("error", json!(null)),
    )
}

/// `POST /register`
pub async fn register(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
    Extension(session): Extension<SessionHandle>,
    InertiaForm(form): InertiaForm<RegisterForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return render_inertia(
                &state,
                &request,
                &presentation,
                authenticated,
                "Register",
                "Create account",
                |props| props.value("error", json!("CSRF token mismatch.")),
            );
        }
    }

    if form.password.trim().len() < 8 {
        return render_inertia(
            &state,
            &request,
            &presentation,
            authenticated,
            "Register",
            "Create account",
            |props| props.value("error", json!("Password must be at least 8 characters.")),
        );
    }

    if form.password != form.password_confirmation {
        return render_inertia(
            &state,
            &request,
            &presentation,
            authenticated,
            "Register",
            "Create account",
            |props| props.value("error", json!("Passwords do not match.")),
        );
    }

    let auth = effective_auth(&state);
    let email = form.email.trim();
    let user = match DemoUserStore::register(email, form.password.trim(), &auth.demo.email) {
        Ok(user) => user,
        Err(_) => {
            return render_inertia(
                &state,
                &request,
                &presentation,
                authenticated,
                "Register",
                "Create account",
                |props| props.value("error", json!("That email is already registered.")),
            );
        }
    };

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
                "Hi {},\n\nConfirm your email address:\n\n{verify_url}",
                form.name.trim()
            ),
        ))
        .await?;

    request
        .redirect(Inertia::redirect("/login"))
        .map_err(RuntimeError::Inertia)
}
