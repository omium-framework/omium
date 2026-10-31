//! JSON health check for `/api/v1/health` (API skeleton — no Inertia).

use axum::Json;
use omium_http::HealthResponse;

/// `GET /api/v1/health`
pub async fn show() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        app: String::from("api"),
        environment: String::from("v1"),
    })
}
