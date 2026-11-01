//! Route registration (Laravel `routes/web.php` + `routes/api.php`).

mod api;
mod manifest;

use std::path::PathBuf;

use axum::Router;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::{delete, get, post};
use omium_http::{
    PresentationNav, inject_request_presentation, share_csrf_token, start_session,
    verify_csrf_header, SessionMiddlewareState,
};
use omium_inertia::inertia_rs::{SharedProps, VersionLayer};
use tower_http::services::ServeDir;

use crate::app::{ASSET_VERSION, MAIN_NAV, RuntimeState};
use crate::controllers::{about, auth, dashboard, examples};
use crate::middleware;
use crate::middleware::inject_authenticated;

pub use manifest::print_route_table;

fn presentation_nav() -> PresentationNav {
    PresentationNav {
        entries: MAIN_NAV,
    }
}

fn web_group(
    routes: Router<RuntimeState>,
    session: SessionMiddlewareState,
) -> Router<RuntimeState> {
    routes
        .route_layer(from_fn(middleware::web_group))
        .route_layer(from_fn(inject_authenticated))
        .route_layer(from_fn(verify_csrf_header))
        .route_layer(from_fn(share_csrf_token))
        .route_layer(from_fn_with_state(
            presentation_nav(),
            inject_request_presentation,
        ))
        .route_layer(from_fn_with_state(session, start_session))
}

fn api_group<S>(routes: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    routes.route_layer(from_fn(middleware::api_group))
}

fn web_routes() -> Router<RuntimeState> {
    Router::new()
        .route("/", get(dashboard::show))
        .route("/login", get(auth::show_login).post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/examples", get(examples::index))
        .route(
            "/examples/new",
            get(examples::create_form).post(examples::create),
        )
        .route(
            "/examples/{id}/edit",
            get(examples::edit_form).post(examples::update),
        )
        .route("/examples/{id}", delete(examples::destroy))
        .route("/about", get(about::show))
}

/// Registers all HTTP routes for the application.
pub fn register(state: RuntimeState) -> Router {
    let session = state.session.clone();
    let public_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("public");
    let app_name = state.app.config().app.name.clone();

    Router::new()
        .merge(web_group(web_routes(), session))
        .nest("/api/v1", api_group(api::routes()))
        .nest_service("/build", ServeDir::new(public_dir.join("build")))
        .with_state(state)
        .layer(axum::Extension(
            SharedProps::new().value("appName", app_name),
        ))
        .layer(VersionLayer::new(ASSET_VERSION))
}
