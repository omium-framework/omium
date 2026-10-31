//! Shared Axum state (config, database, cached Vite tags).

use std::sync::Arc;

use omium_core::state::AppState;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct RuntimeState {
    pub app: AppState,
    pub db: Option<DatabaseConnection>,
    pub vite_tags: Arc<String>,
}
