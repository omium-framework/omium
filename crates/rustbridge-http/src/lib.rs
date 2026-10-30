//! HTTP transport adapter (`Axum` + `Tower`).

mod error;
mod middleware;
mod router;
mod routes;
mod server;

pub use error::{HttpError, ServeError};
pub use middleware::{REQUEST_ID_HEADER, apply_default_middleware};
pub use router::build_router;
pub use routes::{HealthResponse, health_check, health_response};
pub use server::{default_route_manifest, serve, serve_bound_router};
