//! Password reset (demo — sends via Mailpit when SMTP is configured).

use std::fs;

use axum::extract::{Path, State};
use axum::response::Response;
use axum::Extension;
use omium_auth::{verify_token, SessionHandle};
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::{Inertia, InertiaForm, InertiaRequest};
use omium_mail::OutboundMail;
use serde_json::json;

use crate::app::{DemoUserStore, RuntimeError, RuntimeState, effective_auth, email_is_registered};
use crate::forms::{ForgotPasswordForm, ResetPasswordForm};
use crate::http::render_inertia;
use crate::middleware::Authenticated;

fn persist_demo_password(password: &str) -> Result<(), RuntimeError> {
    fs::create_dir_all("storage/framework")?;
    let contents = format!("password = \"{}\"\n", password.replace('"', "\\\""));
    fs::write("storage/framework/demo_credentials.toml", contents)?;
    Ok(())
}

/// `GET /forgot-password`
pub async fn show_forgot(
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
        "ForgotPassword",
        "Forgot password",
        |props| props.value("status", json!(null)),
    )
}

/// `POST /forgot-password`
pub async fn send_reset_link(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
    Extension(session): Extension<SessionHandle>,
    InertiaForm(form): InertiaForm<ForgotPasswordForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return render_inertia(
                &state,
                &request,
                &presentation,
                authenticated,
                "ForgotPassword",
                "Forgot password",
                |props| props.value("error", json!("CSRF token mismatch.")),
            );
        }
    }

    let email = form.email.trim();
    if email_is_registered(&state, email) {
        let token = state.password_reset.issue(email)?;
        let reset_url = format!("http://127.0.0.1:{}/reset-password/{token}", state.app.config().server.port);
        state
            .mailer
            .send(OutboundMail::new(
                email,
                "Reset your password",
                format!("Open this link to reset your password:\n\n{reset_url}"),
            ))
            .await?;
    }

    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "ForgotPassword",
        "Forgot password",
        |props| {
            props.value(
                "status",
                json!("If that email is registered, a reset link has been sent."),
            )
        },
    )
}

/// `GET /reset-password/{token}`
pub async fn show_reset(
    State(state): State<RuntimeState>,
    Path(token): Path<String>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "ResetPassword",
        "Reset password",
        |props| {
            props
                .value("token", json!(token))
                .value("error", json!(null))
        },
    )
}

/// `POST /reset-password/{token}`
pub async fn reset_password(
    State(state): State<RuntimeState>,
    Path(token): Path<String>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
    Extension(session): Extension<SessionHandle>,
    InertiaForm(form): InertiaForm<ResetPasswordForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return render_inertia(
                &state,
                &request,
                &presentation,
                authenticated,
                "ResetPassword",
                "Reset password",
                |props| {
                    props
                        .value("token", json!(token.clone()))
                        .value("error", json!("CSRF token mismatch."))
                },
            );
        }
    }

    if form.password != form.password_confirmation {
        return render_inertia(
            &state,
            &request,
            &presentation,
            authenticated,
            "ResetPassword",
            "Reset password",
            |props| {
                props
                    .value("token", json!(token.clone()))
                    .value("error", json!("Passwords do not match."))
            },
        );
    }

    let Some(email) = state.password_reset.consume(&token)? else {
        return render_inertia(
            &state,
            &request,
            &presentation,
            authenticated,
            "ResetPassword",
            "Reset password",
            |props| {
                props
                    .value("token", json!(token.clone()))
                    .value("error", json!("This reset link is invalid or expired."))
            },
        );
    };

    let auth = effective_auth(&state);
    let password = form.password.trim();
    if email.eq_ignore_ascii_case(&auth.demo.email) {
        persist_demo_password(password)?;
    } else {
        DemoUserStore::update_password(&email, password)?;
    }

    request
        .redirect(Inertia::redirect("/login"))
        .map_err(RuntimeError::Inertia)
}

/// `GET /email/verify/{token}`
pub async fn verify_email(
    State(state): State<RuntimeState>,
    Path(token): Path<String>,
    request: InertiaRequest,
) -> Result<Response, RuntimeError> {
    let path = if state.email_verification.verify(&token)?.is_some() {
        "/login?verified=1"
    } else {
        "/login?verified=0"
    };
    request
        .redirect(Inertia::redirect(path))
        .map_err(RuntimeError::Inertia)
}
