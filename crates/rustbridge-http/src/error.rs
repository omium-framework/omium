//! HTTP-layer errors and safe client responses.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

/// Application-level HTTP errors mapped to safe responses.
#[derive(Debug, Error)]
pub enum HttpError {
    #[error("resource not found")]
    NotFound,

    #[error("internal server error")]
    Internal,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    message: String,
}

impl HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn public_message(&self) -> &'static str {
        match self {
            Self::NotFound => "Not Found",
            Self::Internal => "Internal Server Error",
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = ErrorBody {
            message: self.public_message().to_string(),
        };
        (status, Json(body)).into_response()
    }
}

/// Errors while binding or running the HTTP server.
#[derive(Debug, Error)]
pub enum ServeError {
    #[error("failed to bind HTTP server to {address}: {source}")]
    Bind {
        address: String,
        source: std::io::Error,
    },

    #[error("HTTP server failed: {0}")]
    Server(String),
}

impl From<std::io::Error> for ServeError {
    fn from(source: std::io::Error) -> Self {
        Self::Server(source.to_string())
    }
}
