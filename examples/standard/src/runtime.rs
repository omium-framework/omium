//! Bootstrap and HTTP server lifecycle (thin — routes live in `routes/`, handlers in `controllers/`).

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use omium_core::bootstrap::{BootstrapOptions, bootstrap};
use omium_http::serve_bound_router;

use crate::app::{RuntimeState, connect_if_enabled};
use crate::middleware;
use crate::routes;

/// Bootstraps config and serves the standard (Askama) application.
///
/// # Errors
///
/// Returns [`RuntimeError`] when bootstrap or HTTP serving fails.
pub async fn run(config_dir: PathBuf) -> Result<(), RuntimeError> {
    let app = bootstrap(&BootstrapOptions::new(config_dir))?;
    let db = connect_if_enabled(app.config()).await?;
    let settings = app.config().server.clone();
    let state = RuntimeState {
        app,
        db,
        vite_tags: Arc::new(String::new()),
    };
    let router = build_router(state);
    serve_bound_router(router, &settings).await?;
    Ok(())
}

fn build_router(state: RuntimeState) -> Router {
    let settings = state.app.config().server.clone();
    let router = routes::register(state);
    middleware::apply(router, &settings)
}

pub use crate::app::RuntimeError;
