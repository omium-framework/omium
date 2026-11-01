//! HTTP server lifecycle.

use std::net::SocketAddr;

use omium_core::config::ServerSettings;
use omium_core::shutdown::wait_for_shutdown_signal;
use omium_core::state::AppState;
use tokio::sync::watch;

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

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    if settings.host == "127.0.0.1" {
        let ipv6 = format!("[::1]:{}", settings.port);
        if let Ok(listener_v6) = tokio::net::TcpListener::bind(&ipv6).await {
            tracing::info!(address = %ipv6, "HTTP server listening (IPv6 loopback for localhost)");
            let router_v6 = router.clone();
            let mut shutdown_v6 = shutdown_rx.clone();
            tokio::spawn(async move {
                let shutdown = async move {
                    while !*shutdown_v6.borrow() {
                        if shutdown_v6.changed().await.is_err() {
                            break;
                        }
                    }
                };
                if let Err(err) = axum::serve(listener_v6, router_v6)
                    .with_graceful_shutdown(shutdown)
                    .await
                {
                    tracing::warn!(error = %err, "IPv6 loopback server stopped");
                }
            });
        }
    }

    log_dev_app_urls(settings);

    let shutdown_main = async move {
        wait_for_shutdown_signal().await;
        let _ = shutdown_tx.send(true);
    };

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_main)
        .await
        .map_err(|err| ServeError::Server(err.to_string()))?;

    tracing::info!("HTTP server stopped");
    Ok(())
}

fn log_dev_app_urls(settings: &ServerSettings) {
    if settings.host == "127.0.0.1" {
        tracing::info!(
            app_url = %format!("http://127.0.0.1:{}", settings.port),
            "Open this URL in the browser (Inertia + session)"
        );
    }
}

fn log_listen_address(addr: SocketAddr) {
    tracing::info!(%addr, "HTTP server listening");
}

/// Returns the default framework HTTP routes for documentation and `route:list`.
#[must_use]
pub fn default_route_manifest() -> &'static [(&'static str, &'static str)] {
    &[("GET", "/health")]
}
