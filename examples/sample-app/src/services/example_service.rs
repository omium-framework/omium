//! Example CRUD operations using `SeaORM`.

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::models::example::{self, Entity as Example};

#[derive(Debug, thiserror::Error)]
pub enum ExampleServiceError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

pub struct ExampleService;

impl ExampleService {
    /// Inserts a new example row.
    ///
    /// # Errors
    ///
    /// Returns [`ExampleServiceError`] when the insert fails.
    pub async fn create(
        connection: &DatabaseConnection,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<example::Model, ExampleServiceError> {
        let active = example::ActiveModel {
            name: Set(name.into()),
            description: Set(description),
            ..Default::default()
        };
        Ok(active.insert(connection).await?)
    }

    /// Finds an example row by unique name.
    ///
    /// # Errors
    ///
    /// Returns [`ExampleServiceError`] when the query fails.
    pub async fn find_by_name(
        connection: &DatabaseConnection,
        name: &str,
    ) -> Result<Option<example::Model>, ExampleServiceError> {
        Ok(Example::find()
            .filter(example::Column::Name.eq(name))
            .one(connection)
            .await?)
    }
}
