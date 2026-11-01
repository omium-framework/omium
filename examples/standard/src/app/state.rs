//! Shared Axum state (config, database, cached Vite tags).

use std::sync::Arc;

use omium_core::state::AppState;
use omium_http::{SessionMiddlewareState, ApiTokenMiddlewareState};
use sea_orm::DatabaseConnection;

use super::AuthSettings;

#[derive(Clone)]
pub struct RuntimeState {
    pub app: AppState,
    pub db: Option<DatabaseConnection>,
    pub vite_tags: Arc<String>,
    pub auth: AuthSettings,
    pub session: SessionMiddlewareState,
    pub api_tokens: ApiTokenMiddlewareState,
}
