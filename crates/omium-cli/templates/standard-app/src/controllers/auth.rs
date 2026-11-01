//! Login, logout (demo credentials from `config/auth.toml`).

use axum::extract::State;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Extension;
use axum::Form;
use omium_auth::{regenerate_token, verify_token, SessionHandle};
use omium_core::presentation::RequestPresentation;

use crate::app::{RuntimeError, RuntimeState, verify_credentials};
use crate::forms::LoginForm;
use crate::forms::CsrfForm;
use crate::http::render_html;
use crate::views::LoginTemplate;

/// `GET /login`
pub async fn show_login(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
) -> Result<Response, RuntimeError> {
    let html = render_html(LoginTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        error: None,
    })?;
    Ok(html.into_response())
}

/// `POST /login`
pub async fn login(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(session): Extension<SessionHandle>,
    Form(form): Form<LoginForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return Ok(Html(String::from("CSRF token mismatch")).into_response());
        }
    }

    if let Some(user_id) = verify_credentials(&state, form.email.trim(), &form.password) {
        let mut guard = session.write().await;
        guard.set_user_id(Some(user_id));
        regenerate_token(&mut guard);
        drop(guard);
        return Ok(Redirect::to("/").into_response());
    }

    let html = render_html(LoginTemplate {
        app_name: state.app.config().app.name.clone(),
        nav: presentation.nav,
        flash: presentation.flash,
        csrf_token: presentation.csrf_token,
        error: Some(String::from("Invalid email or password.")),
    })?;
    Ok(html.into_response())
}

/// `POST /logout`
pub async fn logout(
    Extension(session): Extension<SessionHandle>,
    Form(form): Form<CsrfForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return Ok(Html(String::from("CSRF token mismatch")).into_response());
        }
    }
    session.write().await.destroy();
    Ok(Redirect::to("/login").into_response())
}
