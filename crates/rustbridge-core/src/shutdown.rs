//! Graceful shutdown signal handling.

use tracing::info;

/// Waits for `Ctrl+C` or `SIGTERM` (Unix) before returning.
pub async fn wait_for_shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let terminate = match signal(SignalKind::terminate()) {
            Ok(handle) => Some(handle),
            Err(err) => {
                tracing::warn!(error = %err, "SIGTERM handler unavailable; listening for Ctrl+C only");
                None
            }
        };

        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                if let Err(err) = result {
                    tracing::error!(error = %err, "failed to listen for Ctrl+C");
                }
            }
            () = async {
                if let Some(mut terminate) = terminate {
                    let _ = terminate.recv().await;
                } else {
                    std::future::pending::<()>().await;
                }
            } => {}
        }
    }

    #[cfg(not(unix))]
    {
        if let Err(err) = tokio::signal::ctrl_c().await {
            tracing::error!(error = %err, "failed to listen for Ctrl+C");
        }
    }

    info!("shutdown signal received");
}
