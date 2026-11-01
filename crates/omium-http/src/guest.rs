//! Session auth guards (Laravel `auth` / `guest` middleware equivalents).

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use omium_auth::SessionHandle;

static X_INERTIA: &str = "x-inertia";

/// Redirects authenticated users away (login, register, etc.).
pub async fn require_guest(
    Extension(session): Extension<SessionHandle>,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    if session.read().await.user_id().is_some() {
        return guest_redirect("/", &request);
    }
    next.run(request).await
}

/// Redirects guests to sign-in (protected pages).
pub async fn require_auth(
    Extension(session): Extension<SessionHandle>,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    if session.read().await.user_id().is_none() {
        return guest_redirect("/login", &request);
    }
    next.run(request).await
}

fn guest_redirect(location: &str, request: &Request<Body>) -> Response<Body> {
    let status = if request.method() == axum::http::Method::GET {
        StatusCode::FOUND
    } else {
        StatusCode::SEE_OTHER
    };

    if request.headers().contains_key(X_INERTIA) {
        return (
            status,
            [(axum::http::header::LOCATION, location)],
        )
            .into_response();
    }

    axum::response::Redirect::to(location).into_response()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::middleware::from_fn;
    use axum::routing::get;
    use omium_auth::Session;
    use tokio::sync::RwLock;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn guest_middleware_redirects_when_session_has_user() {
        let handle: SessionHandle = Arc::new(RwLock::new(Session::new()));
        handle.write().await.set_user_id(Some(1));

        let app = Router::new()
            .route("/login", get(|| async { "login form" }))
            .route_layer(from_fn(require_guest))
            .layer(Extension(handle));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/login")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert!(
            response.status() == StatusCode::FOUND
                || response.status() == StatusCode::SEE_OTHER
        );
        assert_eq!(
            response
                .headers()
                .get("location")
                .and_then(|value| value.to_str().ok()),
            Some("/")
        );
    }

    #[tokio::test]
    async fn guest_middleware_allows_anonymous_session() {
        let handle: SessionHandle = Arc::new(RwLock::new(Session::new()));

        let app = Router::new()
            .route("/login", get(|| async { "login form" }))
            .route_layer(from_fn(require_guest))
            .layer(Extension(handle));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/login")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), StatusCode::OK);
    }
}
