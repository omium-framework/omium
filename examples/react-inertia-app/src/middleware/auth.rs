//! Injects session auth flag for Inertia shared page props.

use axum::body::Body;
use axum::extract::Extension;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use omium_auth::SessionHandle;

/// Whether the current session has a logged-in user id.
#[derive(Clone, Copy, Debug)]
pub struct Authenticated(pub bool);

pub async fn inject_authenticated(
    Extension(session): Extension<SessionHandle>,
    mut request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let authenticated = session.read().await.user_id().is_some();
    request.extensions_mut().insert(Authenticated(authenticated));
    next.run(request).await
}
