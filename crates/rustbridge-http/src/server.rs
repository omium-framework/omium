//! HTTP server lifecycle.

use std::net::SocketAddr;

use rustbridge_core::config::ServerSettings;
use rustbridge_core::shutdown::wait_for_shutdown_signal;
use rustbridge_core::state::AppState;

use crate::error::ServeError;
use crate::router::build_router;

/// Binds the HTTP server, serves requests, and shuts down gracefully on signal.
///
/// # Errors
///
/// Returns [`ServeError`] when binding or serving fails.
pub async fn serve(state: AppState) -> Result<(), ServeError> {
    let settings = state.config().server.clone();
    let router = build_router(state);
    serve_bound_router(router, &settings).await
}

/// Serves a router that already includes middleware and typed state.
///
/// # Errors
///
/// Returns [`ServeError`] when binding or serving fails.
pub async fn serve_bound_router(
    router: axum::Router,
    settings: &ServerSettings,
) -> Result<(), ServeError> {
    let address = format!("{}:{}", settings.host, settings.port);
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .map_err(|source| ServeError::Bind {
            address: address.clone(),
            source,
        })?;

    let bound = listener.local_addr().map_err(ServeError::from)?;
    log_listen_address(bound);

    axum::serve(listener, router)
        .with_graceful_shutdown(wait_for_shutdown_signal())
        .await
        .map_err(|err| ServeError::Server(err.to_string()))?;

    tracing::info!("HTTP server stopped");
    Ok(())
}

fn log_listen_address(addr: SocketAddr) {
    tracing::info!(%addr, "HTTP server listening");
}

/// Returns the default framework HTTP routes for documentation and `route:list`.
#[must_use]
pub fn default_route_manifest() -> &'static [(&'static str, &'static str)] {
    &[("GET", "/health")]
}
