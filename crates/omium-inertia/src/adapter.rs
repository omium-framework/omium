//! Trait boundary for wrapping a third-party Inertia adapter (D-013 approved: `inertia_rs` bridge).

use axum::response::Response;

/// Framework-owned Inertia port (not implemented for production yet).
pub trait InertiaAdapter: Send + Sync {
    /// Human-readable adapter id (for logging and config), e.g. `inertia_rs` or `inertia-omega`.
    fn adapter_id(&self) -> &'static str;

    /// Renders a page component with serializable props into an HTTP response.
    ///
    /// # Errors
    ///
    /// Implementations return an error when prop serialization or protocol handling fails.
    fn render_page(
        &self,
        component: &str,
        props: serde_json::Value,
    ) -> Result<Response, InertiaAdapterError>;
}

/// Errors from the Inertia adapter boundary.
#[derive(Debug, thiserror::Error)]
pub enum InertiaAdapterError {
    #[error("failed to render Inertia page")]
    Render(String),
}
