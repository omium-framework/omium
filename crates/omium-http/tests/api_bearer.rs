use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode, header};
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::get;
use axum::Json;
use http_body_util::BodyExt;
use omium_auth::{ApiTokenUser, ConfigTokenStore};
use omium_http::{authenticate_api_token, require_api_token, ApiTokenMiddlewareState};
use tower::ServiceExt;

#[tokio::test]
async fn valid_bearer_token_reaches_protected_handler() {
    let token_state = ApiTokenMiddlewareState::new(
        true,
        Arc::new(ConfigTokenStore::new("omium-dev-api-token", 7)),
    );

    let app = Router::new()
        .route(
            "/me",
            get(|Extension(user): Extension<ApiTokenUser>| async move {
                Json(serde_json::json!({ "userId": user.user_id }))
            }),
        )
        .route_layer(from_fn(require_api_token))
        .route_layer(from_fn_with_state(
            token_state,
            authenticate_api_token,
        ));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me")
                .header(
                    header::AUTHORIZATION,
                    "Bearer omium-dev-api-token",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["userId"], 7);
}

#[tokio::test]
async fn missing_bearer_on_protected_route_returns_unauthorized() {
    let token_state = ApiTokenMiddlewareState::new(
        true,
        Arc::new(ConfigTokenStore::new("omium-dev-api-token", 1)),
    );

    let app = Router::new()
        .route("/me", get(|| async { "ok" }))
        .route_layer(from_fn(require_api_token))
        .route_layer(from_fn_with_state(
            token_state,
            authenticate_api_token,
        ));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn invalid_bearer_returns_unauthorized() {
    let token_state = ApiTokenMiddlewareState::new(
        true,
        Arc::new(ConfigTokenStore::new("omium-dev-api-token", 1)),
    );

    let app = Router::new()
        .route("/me", get(|| async { "ok" }))
        .route_layer(from_fn(require_api_token))
        .route_layer(from_fn_with_state(
            token_state,
            authenticate_api_token,
        ));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/me")
                .header(header::AUTHORIZATION, "Bearer wrong-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
