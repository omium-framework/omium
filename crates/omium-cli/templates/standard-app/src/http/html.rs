//! Server-rendered HTML (Askama) helpers.

use axum::response::Html;
use omium_view::into_html;

use crate::app::RuntimeError;

/// Renders an Askama template into an Axum HTML response.
///
/// # Errors
///
/// Returns [`RuntimeError`] when template rendering fails.
pub fn render_html<T: askama::Template>(template: T) -> Result<Html<String>, RuntimeError> {
    into_html(template).map_err(RuntimeError::View)
}
