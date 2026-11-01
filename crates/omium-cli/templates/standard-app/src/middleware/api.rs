//! API route middleware group (attach via `api_group` in `routes/mod.rs`).
//!
//! Laravel `Route::middleware('api')->prefix('api/v1')->group(...)` equivalent.
//! CSRF, CORS, throttling, and auth are not implemented here yet (require approved design).
//! Bearer token auth runs via `authenticate_api_token` in `routes/mod.rs` (see `config/auth.toml`).

use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Runs before JSON API handlers (stateless; no Inertia session/CSRF stack).
pub async fn api_group(request: Request<Body>, next: Next) -> Response {
    tracing::debug!(
        method = %request.method(),
        uri = %request.uri(),
        "api middleware group"
    );
    next.run(request).await
}
