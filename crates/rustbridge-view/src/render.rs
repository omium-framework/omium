//! Helpers to turn Askama templates into HTTP responses.

use askama::Template;
use axum::response::Html;

use crate::error::ViewError;

/// Renders a template into an Axum HTML response.
///
/// # Errors
///
/// Returns [`ViewError`] when template rendering fails.
pub fn into_html<T: Template>(template: T) -> Result<Html<String>, ViewError> {
    Ok(Html(template.render()?))
}
