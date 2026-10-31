//! Form body extraction for Inertia visits and classic HTML forms.

use axum::extract::{Form, FromRequest, Json, Request};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;

/// Parsed form fields from either an Inertia JSON visit or `application/x-www-form-urlencoded`.
pub struct InertiaForm<T>(pub T);

impl<T, S> FromRequest<S> for InertiaForm<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let is_json = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| {
                value
                    .split(';')
                    .next()
                    .is_some_and(|mime| mime.trim().eq_ignore_ascii_case("application/json"))
            });

        if is_json {
            match Json::<T>::from_request(req, state).await {
                Ok(Json(inner)) => Ok(InertiaForm(inner)),
                Err(rejection) => Err(rejection.into_response()),
            }
        } else {
            match Form::<T>::from_request(req, state).await {
                Ok(Form(inner)) => Ok(InertiaForm(inner)),
                Err(rejection) => Err(rejection.into_response()),
            }
        }
    }
}
