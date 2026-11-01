//! CSRF middleware (session synchronizer token).

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{HeaderName, Method, Request, Response, StatusCode};
use axum::middleware::Next;
use omium_auth::{SessionHandle, ensure_token, verify_token};

static X_XSRF_TOKEN: HeaderName = HeaderName::from_static("x-xsrf-token");

/// Writes the CSRF token into [`omium_core::presentation::RequestPresentation`] when present.
pub async fn share_csrf_token(
    Extension(session): Extension<SessionHandle>,
    mut request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let token = {
        let mut guard = session.write().await;
        ensure_token(&mut guard)
    };

    if let Some(presentation) = request.extensions_mut().get_mut::<omium_core::presentation::RequestPresentation>() {
        presentation.csrf_token = Some(token);
    }

    next.run(request).await
}

/// Rejects state-changing requests when `X-XSRF-TOKEN` is present but invalid.
///
/// HTML form posts should verify `_token` in the handler via [`verify_token`].
pub async fn verify_csrf_header(
    Extension(session): Extension<SessionHandle>,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    if !is_state_changing(request.method()) {
        return next.run(request).await;
    }

    if let Some(header) = request.headers().get(&X_XSRF_TOKEN) {
        if let Ok(submitted) = header.to_str() {
            let valid = {
                let guard = session.read().await;
                verify_token(&guard, submitted)
            };
            if !valid {
                return csrf_failed_response();
            }
        }
    }

    next.run(request).await
}

fn is_state_changing(method: &Method) -> bool {
    matches!(
        method,
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE
    )
}

fn csrf_failed_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("content-type", "text/plain; charset=utf-8")
        .body(Body::from("CSRF token mismatch"))
        .unwrap_or_else(|_| Response::new(Body::from("CSRF token mismatch")))
}
