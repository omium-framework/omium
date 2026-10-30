//! Application router composition.

use axum::Router;
use axum::routing::get;
use rustbridge_core::state::AppState;

use crate::error::HttpError;
use crate::middleware::apply_default_middleware;
use crate::routes::health_check;

/// Builds the default application router with framework routes and middleware.
pub fn build_router(state: AppState) -> Router {
    let settings = state.config().server.clone();
    let router = Router::new()
        .route("/health", get(health_check))
        .fallback(not_found)
        .with_state(state);

    apply_default_middleware(router, &settings)
}

async fn not_found() -> HttpError {
    HttpError::NotFound
}
