//! Token persistence (config demo store + database driver).

use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use thiserror::Error;

use super::hash_api_token;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("token store database error: {0}")]
    Database(String),
}

/// Lookup of API tokens to user ids.
#[async_trait]
pub trait PersonalAccessTokenStore: Send + Sync {
    async fn find_user_id_by_token(&self, plain_token: &str) -> Result<Option<u64>, TokenError>;
}

/// Demo / local dev: single plaintext token from config (hashed in memory).
#[derive(Clone)]
pub struct ConfigTokenStore {
    token_hash: String,
    user_id: u64,
}

impl ConfigTokenStore {
    #[must_use]
    pub fn new(plain_token: &str, user_id: u64) -> Self {
        Self {
            token_hash: hash_api_token(plain_token),
            user_id,
        }
    }
}

#[async_trait]
impl PersonalAccessTokenStore for ConfigTokenStore {
    async fn find_user_id_by_token(&self, plain_token: &str) -> Result<Option<u64>, TokenError> {
        if hash_api_token(plain_token) == self.token_hash {
            Ok(Some(self.user_id))
        } else {
            Ok(None)
        }
    }
}

/// No-op store when API tokens are disabled.
pub struct DisabledTokenStore;

#[async_trait]
impl PersonalAccessTokenStore for DisabledTokenStore {
    async fn find_user_id_by_token(&self, _plain_token: &str) -> Result<Option<u64>, TokenError> {
        Ok(None)
    }
}

/// `personal_access_tokens` table (app migration must create it).
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "personal_access_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub token: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub abilities: Option<String>,
    pub last_used_at: Option<i64>,
    pub expires_at: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone)]
pub struct DatabasePersonalAccessTokenStore {
    db: DatabaseConnection,
}

impl DatabasePersonalAccessTokenStore {
    #[must_use]
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PersonalAccessTokenStore for DatabasePersonalAccessTokenStore {
    async fn find_user_id_by_token(&self, plain_token: &str) -> Result<Option<u64>, TokenError> {
        let token_hash = hash_api_token(plain_token);
        let row = Entity::find()
            .filter(Column::Token.eq(token_hash))
            .one(&self.db)
            .await
            .map_err(|error| TokenError::Database(error.to_string()))?;

        Ok(row.map(|model| model.user_id as u64))
    }
}

/// Builds a store from demo settings (used by application templates).
#[must_use]
pub fn demo_token_store(plain_token: &str, user_id: u64) -> Arc<dyn PersonalAccessTokenStore> {
    Arc::new(ConfigTokenStore::new(plain_token, user_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn config_store_matches_plain_token() {
        let store = ConfigTokenStore::new("omium-dev-api-token", 1);
        assert_eq!(
            store
                .find_user_id_by_token("omium-dev-api-token")
                .await
                .expect("lookup"),
            Some(1)
        );
        assert_eq!(
            store.find_user_id_by_token("wrong").await.expect("lookup"),
            None
        );
    }
}
