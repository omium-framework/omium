//! Built-in HTTP routes.

use axum::Json;
use axum::extract::State;
use rustbridge_core::state::AppState;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub app: String,
    pub environment: String,
}

/// `GET /health` readiness endpoint.
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    health_response(&state)
}

pub fn health_response(state: &AppState) -> Json<HealthResponse> {
    let config = state.config();
    Json(HealthResponse {
        status: "ok",
        app: config.app.name.clone(),
        environment: config.app.environment.clone(),
    })
}
