//! Shared application state constructed at bootstrap.

use std::sync::Arc;

use crate::config::AppConfig;

/// Immutable application state injected into handlers (Phase 2+).
#[derive(Clone)]
pub struct AppState {
    config: Arc<AppConfig>,
}

impl AppState {
    #[must_use]
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[must_use]
    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
