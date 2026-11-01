//! Dashboard page.

use axum::extract::State;
use axum::response::Response;
use axum::Extension;
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::InertiaRequest;

use crate::app::{RuntimeError, RuntimeState};
use crate::http::render_inertia;
use crate::middleware::Authenticated;

/// `GET /`
pub async fn show(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    let app_name = state.app.config().app.name.clone();
    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "Dashboard",
        "Welcome",
        |props| {
            props
                .value("greeting", format!("Welcome to {app_name}"))
                .value(
                    "hint",
                    "Use the navigation above — pages swap without a full browser reload.",
                )
                .value("modeLabel", "Vue + Inertia")
        },
    )
}
