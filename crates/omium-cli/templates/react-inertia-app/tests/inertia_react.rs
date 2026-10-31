use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use inertia_rs::headers::X_INERTIA;
use inertia_react_app::runtime;
use std::path::PathBuf;
use tower::ServiceExt;

#[tokio::test]
async fn dashboard_supports_html_and_inertia_json() {
    let config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config");
    let app = runtime::build_router_for_test(config_dir).expect("router");

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
                .header("X-Inertia-Version", "omium-react-1")
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
}
