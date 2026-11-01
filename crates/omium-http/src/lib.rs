//! HTTP transport adapter (`Axum` + `Tower`).

mod api_token;
mod csrf;
mod error;
mod guest;
mod middleware;
mod presentation;
mod router;
mod routes;
mod server;
mod session;

pub use error::{HttpError, ServeError};
pub use middleware::{REQUEST_ID_HEADER, apply_default_middleware};
pub use api_token::{
    authenticate_api_token, require_api_token, ApiTokenMiddlewareState,
};
pub use csrf::{share_csrf_token, verify_csrf_header};
pub use guest::{require_auth, require_guest};
pub use omium_auth::ApiTokenUser;
pub use presentation::{
    PresentationNav, apply_flash_to_response, flash_error_cookie, flash_success_cookie,
    inject_request_presentation,
};
pub use session::{SessionMiddlewareState, start_session};
pub use omium_auth::SessionCookieConfig;
pub use router::build_router;
pub use routes::{HealthResponse, health_check, health_response};
pub use server::{default_route_manifest, serve, serve_bound_router};
