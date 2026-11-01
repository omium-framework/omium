//! About page.

use axum::extract::State;
use axum::response::Response;
use axum::Extension;
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::InertiaRequest;

use crate::app::{RuntimeError, RuntimeState};
use crate::http::render_inertia;
use crate::middleware::Authenticated;

/// `GET /about`
pub async fn show(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    let stack = serde_json::json!([
        "omium-core — config & bootstrap",
        "omium-http — middleware",
        "omium-inertia — Vite tags + Inertia bridge",
        "Vue 3 + @inertiajs/vue3 — UI (Vite build)",
    ]);
    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "About",
        "About",
        |props| props.value("stack", stack),
    )
}
