//! Application middleware (Laravel `app/Http/Middleware` + `Kernel` equivalent).

mod api;
mod web;

use axum::Router;
use omium_core::config::ServerSettings;
use omium_http::apply_default_middleware;

pub use api::api_group;
pub use web::web_group;

/// Applies app middleware, then the framework default stack (request id, trace, timeout, body limit).
pub fn apply<S>(router: Router<S>, settings: &ServerSettings) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    apply_default_middleware(router, settings)
}
