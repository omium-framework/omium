//! Bootstrap and HTTP server lifecycle (thin — routes live in `routes/`, handlers in `controllers/`).

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use omium_core::bootstrap::{BootstrapOptions, bootstrap};
use omium_http::serve_bound_router;

use crate::app::{RuntimeState, connect_if_enabled, resolve_vite_tags};
use crate::middleware;
use crate::routes;

/// Bootstraps config and serves the Vue + Inertia application.
///
/// # Errors
///
/// Returns [`RuntimeError`] when bootstrap, Vite resolution, or HTTP serving fails.
pub async fn run(config_dir: PathBuf) -> Result<(), RuntimeError> {
    let app = bootstrap(&BootstrapOptions::new(config_dir))?;
    let db = connect_if_enabled(app.config()).await?;
    let vite_tags = Arc::new(resolve_vite_tags(app.config())?);
    let settings = app.config().server.clone();
    let state = RuntimeState {
        app,
        db,
        vite_tags,
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
    let app = bootstrap(&BootstrapOptions::new(config_dir))?;
    let vite_tags = Arc::new(resolve_vite_tags(app.config())?);
    Ok(build_router(RuntimeState {
        app,
        db: None,
        vite_tags,
    }))
}

fn build_router(state: RuntimeState) -> Router {
    let settings = state.app.config().server.clone();
    let router = routes::register(state);
    middleware::apply(router, &settings)
}

pub use crate::app::RuntimeError;
