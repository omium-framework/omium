//! Application error type mapped to safe HTTP responses.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use omium_core::error::BootstrapError;
use omium_db::DatabaseError;
use omium_http::ServeError;
use omium_view::ViewError;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Bootstrap(#[from] BootstrapError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Http(#[from] ServeError),

    #[error(transparent)]
    View(#[from] ViewError),

    #[error(transparent)]
    ExampleService(#[from] crate::services::ExampleServiceError),

    #[error(transparent)]
    Mail(#[from] omium_mail::MailError),

    #[error(transparent)]
    AuthStore(#[from] omium_auth::SessionError),

    #[error(transparent)]
    DemoUser(#[from] crate::app::DemoUserError),
}

impl IntoResponse for RuntimeError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "request failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error occurred while processing your request.",
        )
            .into_response()
    }
}
