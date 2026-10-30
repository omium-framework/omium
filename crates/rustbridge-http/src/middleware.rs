//! Default Tower middleware stack.

use std::time::Duration;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderName, StatusCode};
use rustbridge_core::config::ServerSettings;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Applies request ID propagation, tracing, timeout, and body size limits.
pub fn apply_default_middleware<S>(router: Router<S>, settings: &ServerSettings) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    router
        .layer(PropagateRequestIdLayer::new(request_id.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    let request_id = request
                        .headers()
                        .get(REQUEST_ID_HEADER)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("-");
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        uri = %request.uri(),
                        request_id = request_id,
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            status = response.status().as_u16(),
                            latency_ms = latency.as_millis(),
                            "request completed"
                        );
                    },
                ),
        )
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(settings.request_timeout_secs),
        ))
        .layer(DefaultBodyLimit::max(settings.body_limit_bytes))
        .layer(SetRequestIdLayer::new(request_id, MakeRequestUuid))
}
