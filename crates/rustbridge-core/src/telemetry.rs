//! Structured logging initialization.

use tracing_subscriber::EnvFilter;

use crate::config::LoggingSettings;
use crate::error::BootstrapError;

/// Initializes the global `tracing` subscriber from configuration.
///
/// # Errors
///
/// Returns an error when the log level filter cannot be constructed.
pub fn init_tracing(settings: &LoggingSettings) -> Result<(), BootstrapError> {
    let directive = format!("rustbridge={},tower_http=info", settings.level);
    let env_filter = EnvFilter::try_new(&directive)
        .map_err(|err| BootstrapError::TracingInit(err.to_string()))?;

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|err| BootstrapError::TracingInit(err.to_string()))?;

    Ok(())
}
