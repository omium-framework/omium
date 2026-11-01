//! Route registration (Laravel `routes/web.php` + `routes/api.php`).

mod api;
mod manifest;

use axum::Router;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::routing::{get, post};
use omium_http::{
    PresentationNav, authenticate_api_token, inject_request_presentation, require_guest,
    share_csrf_token, start_session, verify_csrf_header, ApiTokenMiddlewareState,
    SessionMiddlewareState,
};

use crate::app::{MAIN_NAV, RuntimeState};
use crate::controllers::{auth, password, register, welcome};
use crate::middleware;

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
        .route_layer(from_fn(verify_csrf_header))
        .route_layer(from_fn(share_csrf_token))
        .route_layer(from_fn_with_state(
            presentation_nav(),
            inject_request_presentation,
        ))
        .route_layer(from_fn_with_state(session, start_session))
}

fn api_group(
    routes: Router<RuntimeState>,
    api_tokens: ApiTokenMiddlewareState,
) -> Router<RuntimeState> {
    routes
        .route_layer(from_fn(middleware::api_group))
        .route_layer(from_fn_with_state(
            api_tokens,
            authenticate_api_token,
        ))
}

fn guest_routes() -> Router<RuntimeState> {
    Router::new()
        .route("/login", get(auth::show_login).post(auth::login))
        .route(
            "/register",
            get(register::show_register).post(register::register),
        )
        .route_layer(from_fn(require_guest))
}

fn web_routes() -> Router<RuntimeState> {
    Router::new()
        .merge(guest_routes())
        .route("/", get(welcome::show))
        .route("/logout", post(auth::logout))
        .route(
            "/forgot-password",
            get(password::show_forgot).post(password::send_reset_link),
        )
        .route(
            "/reset-password/{token}",
            get(password::show_reset).post(password::reset_password),
        )
        .route("/email/verify/{token}", get(password::verify_email))
        .route(
            "/email/verification-notification",
            post(password::resend_verification),
        )
}

/// Registers all HTTP routes for the application.
pub fn register(state: RuntimeState) -> Router {
    let session = state.session.clone();
    let api_tokens = state.api_tokens.clone();
    Router::new()
        .merge(web_group(web_routes(), session))
        .nest("/api/v1", api_group(api::routes(), api_tokens))
        .with_state(state)
}