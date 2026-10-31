//! Example CRUD operations using `SeaORM`.

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait, Set};

use crate::models::example::{self, Entity as Example};

#[derive(Debug, thiserror::Error)]
pub enum ExampleServiceError {
    #[error(transparent)]
    Database(#[from] sea_orm::DbErr),
}

pub struct ExampleService;

impl ExampleService {
    /// Lists all example rows for the Examples Inertia page.
    ///
    /// # Errors
    ///
    /// Returns [`ExampleServiceError`] when the query fails.
    pub async fn list(
        connection: &DatabaseConnection,
    ) -> Result<Vec<example::Model>, ExampleServiceError> {
        Ok(Example::find().all(connection).await?)
    }

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

    /// Finds an example row by primary key.
    ///
    /// # Errors
    ///
    /// Returns [`ExampleServiceError`] when the query fails.
    pub async fn find_by_id(
        connection: &DatabaseConnection,
        id: u32,
    ) -> Result<Option<example::Model>, ExampleServiceError> {
        Ok(Example::find_by_id(id).one(connection).await?)
    }

    /// Updates an existing example row.
    ///
    /// # Errors
    ///
    /// Returns [`ExampleServiceError`] when the update fails.
    pub async fn update(
        connection: &DatabaseConnection,
        id: u32,
        name: impl Into<String>,
        description: Option<String>,
    ) -> Result<example::Model, ExampleServiceError> {
        let existing = Example::find_by_id(id)
            .one(connection)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("Example {id} not found")))?;

        let mut active: example::ActiveModel = existing.into();
        active.name = Set(name.into());
        active.description = Set(description);
        Ok(active.update(connection).await?)
    }

    /// Deletes an example row by primary key.
    ///
    /// # Errors
    ///
    /// Returns [`ExampleServiceError`] when the delete fails.
    pub async fn delete(
        connection: &DatabaseConnection,
        id: u32,
    ) -> Result<(), ExampleServiceError> {
        let existing = Example::find_by_id(id)
            .one(connection)
            .await?
            .ok_or_else(|| sea_orm::DbErr::RecordNotFound(format!("Example {id} not found")))?;

        existing.delete(connection).await?;
        Ok(())
    }
}
