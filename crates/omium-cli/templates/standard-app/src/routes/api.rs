//! API routes (`routes/api.php` skeleton) — register under `/api/v1` in `routes/mod.rs`.
//!
//! Keep [`ENTRIES`] in sync with [`routes`] for `./omnisan route:list`.

use axum::Router;
use axum::routing::get;

use crate::app::RuntimeState;
use crate::controllers::api::health;
use crate::routes::manifest::RouteEntry;

pub const ENTRIES: &[RouteEntry] = &[RouteEntry {
    methods: "GET",
    uri: "/api/v1/health",
    action: "api::health::show",
}];

/// Routes nested at `/api/v1` (paths here are relative to that prefix).
pub fn routes() -> Router<RuntimeState> {
    Router::new().route("/health", get(health::show))
}
