//! Application root JSON (`GET /`).

use axum::Extension;
use axum::Json;
use axum::extract::State;
use omium_core::presentation::RequestPresentation;
use serde_json::json;

use crate::app::RuntimeState;

/// `GET /`
pub async fn show(
    State(state): State<RuntimeState>,
    Extension(presentation): Extension<RequestPresentation>,
) -> Json<serde_json::Value> {
    Json(json!({
        "app": state.app.config().app.name,
        "mode": "api",
        "currentPath": presentation.current_path,
        "nav": presentation.nav,
        "flash": presentation.flash,
    }))
}
