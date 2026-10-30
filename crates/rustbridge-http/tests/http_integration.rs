use axum::Router;
use axum::body::Body;
use axum::body::Bytes;
use axum::http::{Request, StatusCode};
use axum::routing::post;
use http_body_util::BodyExt;
use rustbridge_core::config::AppConfig;
use rustbridge_core::state::AppState;
use rustbridge_http::{REQUEST_ID_HEADER, apply_default_middleware, build_router};
use tower::ServiceExt;

fn test_state(body_limit_bytes: usize) -> AppState {
    let mut config = AppConfig::default();
    config.app.name = String::from("Test App");
    config.server.port = 8080;
    config.server.body_limit_bytes = body_limit_bytes;
    AppState::new(config)
}

#[tokio::test]
async fn health_endpoint_returns_ok_payload() {
    let app = build_router(test_state(1024));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .get(REQUEST_ID_HEADER)
            .is_some_and(|value| !value.is_empty())
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["status"], "ok");
    assert_eq!(json["app"], "Test App");
}

#[tokio::test]
async fn unknown_route_returns_safe_not_found_json() {
    let app = build_router(test_state(1024));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/missing")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(json["message"], "Not Found");
}

async fn accept_body(body: Bytes) -> StatusCode {
    let _ = body;
    StatusCode::NO_CONTENT
}

#[tokio::test]
async fn oversized_request_body_is_rejected() {
    let state = test_state(16);
    let router = Router::new()
        .route("/accept", post(accept_body))
        .with_state(state.clone());
    let app = apply_default_middleware(router, &state.config().server);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/accept")
                .header("content-type", "application/octet-stream")
                .body(Body::from(vec![0_u8; 32]))
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
