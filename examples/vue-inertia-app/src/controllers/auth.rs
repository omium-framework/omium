//! Login and logout (demo credentials from `config/auth.toml`).

use axum::extract::State;
use axum::response::Response;
use axum::Extension;
use omium_auth::{regenerate_token, verify_token, SessionHandle};
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::{Inertia, InertiaForm, InertiaRequest};
use serde_json::json;

use crate::app::{RuntimeError, RuntimeState, verify_credentials};
use crate::forms::LoginForm;
use crate::http::render_inertia;
use crate::middleware::Authenticated;

/// `GET /login`
pub async fn show_login(
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
        "Login",
        "Sign in",
        |props| props.value("error", json!(null)),
    )
}

/// `POST /login`
pub async fn login(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
    Extension(session): Extension<SessionHandle>,
    InertiaForm(form): InertiaForm<LoginForm>,
) -> Result<Response, RuntimeError> {
    {
        let guard = session.read().await;
        if !verify_token(&guard, &form._token) {
            return render_inertia(
                &state,
                &request,
                &presentation,
                authenticated,
                "Login",
                "Sign in",
                |props| props.value("error", json!("CSRF token mismatch.")),
            );
        }
    }

    if let Some(user_id) = verify_credentials(&state, form.email.trim(), &form.password) {
        let mut guard = session.write().await;
        guard.set_user_id(Some(user_id));
        regenerate_token(&mut guard);
        drop(guard);
        return request
            .redirect(Inertia::redirect("/"))
            .map_err(RuntimeError::Inertia);
    }

    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "Login",
        "Sign in",
        |props| props.value("error", json!("Invalid email or password.")),
    )
}

/// `POST /logout`
pub async fn logout(
    request: InertiaRequest,
    Extension(session): Extension<SessionHandle>,
) -> Result<Response, RuntimeError> {
    session.write().await.destroy();
    request
        .redirect(Inertia::redirect("/login"))
        .map_err(RuntimeError::Inertia)
}
