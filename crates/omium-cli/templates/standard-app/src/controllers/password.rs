//! Password reset and email verification (demo — file stores until A7 `users` table).

use std::fs;

use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Extension;
use axum::Form;
use omium_auth::{verify_token, SessionHandle};
use omium_core::presentation::RequestPresentation;
use omium_mail::OutboundMail;

use crate::app::{DemoUserStore, RuntimeError, RuntimeState, effective_auth, email_is_registered};
use crate::forms::{ForgotPasswordForm, ResetPasswordForm};
use crate::http::render_html;
use crate::views::{ForgotPasswordTemplate, ResetPasswordTemplate};

fn persist_demo_password(password: &str) -> Result<(), RuntimeError> {
    fs::create_dir_all("storage/framework")?;
    let contents = format!("password = \"{}\"\n", password.replace('"', "\\\""));
    fs::write("storage/framework/demo_credentials.toml", contents)?;
    Ok(())
}

/// `GET /forgot-password`
pub async fn show_forgot(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
) -> Result<Response, RuntimeError> {
    let html = render_html(ForgotPasswordTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        status: None,
    })?;
    Ok(html.into_response())
}

/// `POST /forgot-password`
pub async fn send_reset_link(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(session): Extension<SessionHandle>,
    Form(form): Form<ForgotPasswordForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return Ok(Html(String::from("CSRF token mismatch")).into_response());
        }
    }

    let email = form.email.trim();
    if email_is_registered(&state, email) {
        let token = state.password_reset.issue(email)?;
        let reset_url = format!(
            "http://127.0.0.1:{}/reset-password/{token}",
            state.app.config().server.port
        );
        state
            .mailer
            .send(OutboundMail::new(
                email,
                "Reset your password",
                format!("Open this link to reset your password:\n\n{reset_url}"),
            ))
            .await?;
    }

    let html = render_html(ForgotPasswordTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        status: Some(String::from(
            "If that email is registered, a reset link has been sent.",
        )),
    })?;
    Ok(html.into_response())
}

/// `GET /reset-password/{token}`
pub async fn show_reset(
    State(state): State<RuntimeState>,
    Path(token): Path<String>,
    Extension(presentation): Extension<RequestPresentation>,
) -> Result<Response, RuntimeError> {
    let html = render_html(ResetPasswordTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        token,
        error: None,
    })?;
    Ok(html.into_response())
}

/// `POST /reset-password/{token}`
pub async fn reset_password(
    State(state): State<RuntimeState>,
    Path(token): Path<String>,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(session): Extension<SessionHandle>,
    Form(form): Form<ResetPasswordForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return Ok(Html(String::from("CSRF token mismatch")).into_response());
        }
    }

    if form.password != form.password_confirmation {
        let html = render_html(ResetPasswordTemplate {
            app_name: state.app.config().app.name.clone(),
            nav: presentation.nav,
            flash: presentation.flash,
            csrf_token: presentation.csrf_token,
            token: token.clone(),
            error: Some(String::from("Passwords do not match.")),
        })?;
        return Ok(html.into_response());
    }

    let Some(email) = state.password_reset.consume(&token)? else {
        let html = render_html(ResetPasswordTemplate {
            app_name: state.app.config().app.name.clone(),
            nav: presentation.nav,
            flash: presentation.flash,
            csrf_token: presentation.csrf_token,
            token: token.clone(),
            error: Some(String::from("This reset link is invalid or expired.")),
        })?;
        return Ok(html.into_response());
    };

    let auth = effective_auth(&state);
    let password = form.password.trim();
    if email.eq_ignore_ascii_case(&auth.demo.email) {
        persist_demo_password(password)?;
    } else {
        DemoUserStore::update_password(&email, password)?;
    }
    Ok(Redirect::to("/login").into_response())
}

/// `GET /email/verify/{token}`
pub async fn verify_email(
    State(state): State<RuntimeState>,
    Path(token): Path<String>,
) -> Result<Response, RuntimeError> {
    let message = if state.email_verification.verify(&token)?.is_some() {
        "Email verified. You can sign in."
    } else {
        "Verification link is invalid or expired."
    };
    Ok(Html(format!("<p>{message}</p>")).into_response())
}

/// `POST /email/verification-notification` (resend for demo user).
pub async fn resend_verification(
    State(state): State<RuntimeState>,
    Extension(session): Extension<SessionHandle>,
    Extension(presentation): Extension<RequestPresentation>,
    Form(form): Form<crate::forms::CsrfForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return Ok(Html(String::from("CSRF token mismatch")).into_response());
        }
    }

    let email = state.auth.demo.email.clone();
    if !state.email_verification.is_verified(&email)? {
        let token = state.email_verification.issue(&email)?;
        let verify_url = format!("/email/verify/{token}");
        state
            .mailer
            .send(OutboundMail::new(
                &email,
                "Verify your email",
                format!("Confirm your email: {verify_url}"),
            ))
            .await?;
    }

    let html = render_html(ForgotPasswordTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        status: Some(String::from("Verification email sent.")),
    })?;
    Ok(html.into_response())
}
