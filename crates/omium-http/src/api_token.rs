//! Bearer personal access token middleware.

use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use omium_auth::{
    parse_bearer_authorization, ApiTokenUser, PersonalAccessTokenStore, TokenError,
};
use serde::Serialize;

/// Shared state for [`authenticate_api_token`].
#[derive(Clone)]
pub struct ApiTokenMiddlewareState {
    pub enabled: bool,
    pub store: Arc<dyn PersonalAccessTokenStore>,
}

impl ApiTokenMiddlewareState {
    #[must_use]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            store: Arc::new(omium_auth::DisabledTokenStore),
        }
    }

    #[must_use]
    pub fn new(enabled: bool, store: Arc<dyn PersonalAccessTokenStore>) -> Self {
        Self { enabled, store }
    }
}

/// Validates `Authorization: Bearer` when present and attaches [`ApiTokenUser`].
pub async fn authenticate_api_token(
    State(state): State<ApiTokenMiddlewareState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    if !state.enabled {
        return next.run(request).await;
    }

    let Some(header_value) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    else {
        return next.run(request).await;
    };

    let Some(plain) = parse_bearer_authorization(header_value) else {
        return api_unauthorized().into_response();
    };

    match state.store.find_user_id_by_token(plain).await {
        Ok(Some(user_id)) => {
            request.extensions_mut().insert(ApiTokenUser { user_id });
            next.run(request).await
        }
        Ok(None) => api_unauthorized().into_response(),
        Err(TokenError::Database(_)) => api_unauthorized().into_response(),
    }
}

/// Requires a validated [`ApiTokenUser`] (use on protected API routes).
pub async fn require_api_token(request: Request<Body>, next: Next) -> Response {
    if request.extensions().get::<ApiTokenUser>().is_some() {
        next.run(request).await
    } else {
        api_unauthorized().into_response()
    }
}

#[derive(Serialize)]
struct ApiErrorBody {
    message: &'static str,
}

fn api_unauthorized() -> (StatusCode, Json<ApiErrorBody>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiErrorBody {
            message: "Unauthorized",
        }),
    )
}
