//! Database session driver (`sessions` table).

use std::collections::HashMap;

use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde_json::Value;

use super::Session;
use super::SessionId;
use super::store::SessionStore;
use crate::SessionError;

/// `SeaORM` entity for the `sessions` table (app migration must create it).
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: Option<i64>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub payload: String,
    pub last_activity: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone)]
pub struct DatabaseSessionStore {
    db: DatabaseConnection,
}

impl DatabaseSessionStore {
    #[must_use]
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionStore for DatabaseSessionStore {
    async fn load(&self, id: &SessionId) -> Result<Option<Session>, SessionError> {
        let row = Entity::find_by_id(id.as_str().to_string())
            .one(&self.db)
            .await
            .map_err(|e| SessionError::Database(e.to_string()))?;

        let Some(model) = row else {
            return Ok(None);
        };

        let id = SessionId::parse(&model.id)?;
        let data: HashMap<String, Value> = serde_json::from_str(&model.payload)?;
        Ok(Some(Session::from_persisted(
            id,
            model.user_id,
            model.ip_address,
            model.user_agent,
            data,
            model.last_activity,
        )))
    }

    async fn save(&self, session: &Session) -> Result<(), SessionError> {
        if session.is_destroyed() {
            return self.destroy(session.id()).await;
        }

        let payload = serde_json::to_string(session.data_map())?;
        let active = ActiveModel {
            id: Set(session.id().as_str().to_string()),
            user_id: Set(session.user_id()),
            ip_address: Set(session.ip_address().map(str::to_string)),
            user_agent: Set(session.user_agent().map(str::to_string)),
            payload: Set(payload),
            last_activity: Set(session.last_activity()),
        };

        Entity::insert(active)
            .on_conflict(
                OnConflict::column(Column::Id)
                    .update_columns([
                        Column::UserId,
                        Column::IpAddress,
                        Column::UserAgent,
                        Column::Payload,
                        Column::LastActivity,
                    ])
                    .to_owned(),
            )
            .exec_without_returning(&self.db)
            .await
            .map_err(|e| SessionError::Database(e.to_string()))?;
        Ok(())
    }

    async fn destroy(&self, id: &SessionId) -> Result<(), SessionError> {
        Entity::delete_by_id(id.as_str().to_string())
            .exec(&self.db)
            .await
            .map_err(|e| SessionError::Database(e.to_string()))?;
        Ok(())
    }
}
