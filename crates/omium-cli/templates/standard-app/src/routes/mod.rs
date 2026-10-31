//! Route registration (Laravel `routes/web.php` + `routes/api.php`).

mod api;
mod manifest;

use axum::Router;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::get;
use omium_http::{PresentationNav, inject_request_presentation};

use crate::app::{MAIN_NAV, RuntimeState};
use crate::controllers::welcome;
use crate::middleware;

pub use manifest::print_route_table;

fn presentation_nav() -> PresentationNav {
    PresentationNav {
        entries: MAIN_NAV,
    }
}

fn web_group<S>(routes: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    routes
        .route_layer(from_fn_with_state(
            presentation_nav(),
            inject_request_presentation,
        ))
        .route_layer(from_fn(middleware::web_group))
}

fn api_group<S>(routes: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    routes.route_layer(from_fn(middleware::api_group))
}

fn web_routes() -> Router<RuntimeState> {
    Router::new().route("/", get(welcome::show))
}

/// Registers all HTTP routes for the application.
pub fn register(state: RuntimeState) -> Router {
    Router::new()
        .merge(web_group(web_routes()))
        .nest("/api/v1", api_group(api::routes()))
        .with_state(state)
}
