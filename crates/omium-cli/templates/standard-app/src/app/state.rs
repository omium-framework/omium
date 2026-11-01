//! Shared Axum state (config, database, cached Vite tags).

use std::sync::Arc;

use omium_auth::{FileEmailVerificationStore, FilePasswordResetStore};
use omium_core::state::AppState;
use omium_http::{ApiTokenMiddlewareState, SessionMiddlewareState};
use omium_mail::SharedMailer;
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
    pub mailer: SharedMailer,
    pub password_reset: FilePasswordResetStore,
    pub email_verification: FileEmailVerificationStore,
}
