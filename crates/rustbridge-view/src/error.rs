//! View rendering errors.

#[derive(Debug, thiserror::Error)]
pub enum ViewError {
    #[error("failed to render template")]
    Render(#[from] askama::Error),
}
