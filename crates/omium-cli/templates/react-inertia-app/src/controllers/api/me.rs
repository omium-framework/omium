//! Authenticated API user profile (`GET /api/v1/me`).

use axum::Extension;
use axum::Json;
use omium_auth::ApiTokenUser;
use serde::Serialize;

#[derive(Serialize)]
pub struct MeResponse {
    pub user_id: u64,
}

/// `GET /api/v1/me` — requires `Authorization: Bearer` (see `config/auth.toml`).
pub async fn show(Extension(user): Extension<ApiTokenUser>) -> Json<MeResponse> {
    Json(MeResponse {
        user_id: user.user_id,
    })
}
