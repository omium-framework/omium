//! API routes (`routes/api.php` skeleton) — register under `/api/v1` in `routes/mod.rs`.
//!
//! Keep [`ENTRIES`] in sync with [`routes`] for `./omnisan route:list`.

use axum::Router;
use axum::middleware::from_fn;
use axum::routing::get;
use omium_http::require_api_token;

use crate::app::RuntimeState;
use crate::controllers::api::{health, me};
use crate::routes::manifest::RouteEntry;

pub const ENTRIES: &[RouteEntry] = &[
    RouteEntry {
        methods: "GET",
        uri: "/api/v1/health",
        action: "api::health::show",
    },
    RouteEntry {
        methods: "GET",
        uri: "/api/v1/me",
        action: "api::me::show",
    },
];

/// Routes nested at `/api/v1` (paths here are relative to that prefix).
pub fn routes() -> Router<RuntimeState> {
    let protected = Router::new()
        .route("/me", get(me::show))
        .route_layer(from_fn(require_api_token));

    Router::new()
        .route("/health", get(health::show))
        .merge(protected)
}
