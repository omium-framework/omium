//! Web route middleware group (attach to routes in `routes/mod.rs`).

use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

/// Logs when a route in the `web` group is hit (demo — replace with auth, CSRF, etc.).
pub async fn web_group(request: Request<Body>, next: Next) -> Response {
    tracing::debug!(
        method = %request.method(),
        uri = %request.uri(),
        "web middleware group"
    );
    next.run(request).await
}
