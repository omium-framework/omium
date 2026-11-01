//! Bootstrap and HTTP server lifecycle (thin — routes live in `routes/`, handlers in `controllers/`).

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use omium_auth::FileSessionStore;
use omium_core::bootstrap::{BootstrapOptions, bootstrap};
use omium_http::{SessionCookieConfig, SessionMiddlewareState, serve_bound_router};

use crate::app::{
    RuntimeState, build_api_token_middleware, build_app_mailer, connect_if_enabled,
    load_auth_settings_with_overrides, resolve_vite_tags,
};
use omium_auth::{FileEmailVerificationStore, FilePasswordResetStore};
use crate::middleware;
use crate::routes;

/// Bootstraps config and serves the Vue + Inertia application.
///
/// # Errors
///
/// Returns [`RuntimeError`] when bootstrap, Vite resolution, or HTTP serving fails.
pub async fn run(config_dir: PathBuf) -> Result<(), RuntimeError> {
    let app = bootstrap(&BootstrapOptions::new(config_dir.clone()))?;
    let db = connect_if_enabled(app.config()).await?;
    let auth = load_auth_settings_with_overrides(&config_dir);
    let api_tokens = build_api_token_middleware(&auth);
    let (_mail_settings, mailer) = build_app_mailer(&config_dir).map_err(RuntimeError::Mail)?;
    let store = Arc::new(FileSessionStore::new("storage/framework/sessions"));
    let session = SessionMiddlewareState::new(store, SessionCookieConfig::default());
    let vite_tags = Arc::new(resolve_vite_tags(app.config())?);
    let settings = app.config().server.clone();
    let state = RuntimeState {
        app,
        db,
        vite_tags,
        auth,
        session,
        api_tokens,
        mailer,
        password_reset: FilePasswordResetStore::new("storage/framework/password_reset"),
        email_verification: FileEmailVerificationStore::new("storage/framework/email_verification"),
    };
    let router = build_router(state);
    serve_bound_router(router, &settings).await?;
    Ok(())
}

/// Builds the router for integration tests (does not listen on a socket).
///
/// # Errors
///
/// Returns [`RuntimeError`] when bootstrap or Vite tag resolution fails.
pub fn build_router_for_test(config_dir: PathBuf) -> Result<Router, RuntimeError> {
    let app = bootstrap(&BootstrapOptions::new(config_dir.clone()))?;
    let auth = load_auth_settings_with_overrides(&config_dir);
    let api_tokens = build_api_token_middleware(&auth);
    let (_mail_settings, mailer) = build_app_mailer(&config_dir).map_err(RuntimeError::Mail)?;
    let store = Arc::new(FileSessionStore::new("storage/framework/sessions"));
    let session = SessionMiddlewareState::new(store, SessionCookieConfig::default());
    let vite_tags = Arc::new(resolve_vite_tags(app.config())?);
    Ok(build_router(RuntimeState {
        app,
        db: None,
        vite_tags,
        auth,
        session,
        api_tokens,
        mailer,
        password_reset: FilePasswordResetStore::new("storage/framework/password_reset"),
        email_verification: FileEmailVerificationStore::new("storage/framework/email_verification"),
    }))
}

fn build_router(state: RuntimeState) -> Router {
    let settings = state.app.config().server.clone();
    let router = routes::register(state);
    middleware::apply(router, &settings)
}

pub use crate::app::RuntimeError;
