use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use inertia_rs::headers::X_INERTIA;
use inertia_vue_app::runtime;
use std::path::PathBuf;
use std::sync::OnceLock;
use tower::ServiceExt;

fn test_router() -> Router {
    static ROUTER: OnceLock<Router> = OnceLock::new();
    ROUTER
        .get_or_init(|| {
            let config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config");
            runtime::build_router_for_test(config_dir).expect("router")
        })
        .clone()
}

#[tokio::test]
async fn dashboard_supports_html_and_inertia_json() {
    let app = test_router();

    let html_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(html_response.status(), StatusCode::OK);
    let html_body = html_response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let html = String::from_utf8(html_body.to_vec()).expect("utf8");
    assert!(html.contains("Dashboard"));
    assert!(html.contains("data-page=\"app\""));

    let json_response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .header(X_INERTIA, "true")
                .header("X-Inertia-Version", "omium-vue-1")
                .header(header::ACCEPT, "text/html, application/json")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(json_response.status(), StatusCode::OK);
    let json_body = json_response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let page: serde_json::Value = serde_json::from_slice(&json_body).expect("json");
    assert_eq!(page["component"], "Dashboard");
    assert_eq!(page["props"]["appName"], "Omium");
    assert!(page["props"]["csrfToken"].is_string());
}

#[tokio::test]
async fn login_page_exposes_csrf_token_in_inertia_json() {
    let app = test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/login")
                .header(X_INERTIA, "true")
                .header("X-Inertia-Version", "omium-vue-1")
                .header(header::ACCEPT, "text/html, application/json")
                .body(Body::empty())
                .expect("request"),
        )
        .await
        .expect("response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let page: serde_json::Value = serde_json::from_slice(&body).expect("json");
    assert_eq!(page["component"], "Login");
    assert!(page["props"]["csrfToken"].is_string());
}
