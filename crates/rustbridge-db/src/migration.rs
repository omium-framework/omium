//! Migration execution helpers.

use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;

use crate::error::DatabaseError;

/// Applies all pending migrations for the given migrator.
///
/// Production deployments should call this from an explicit command, not on every boot.
///
/// # Errors
///
/// Returns [`DatabaseError`] when migration execution fails.
pub async fn apply_migrations<M: MigratorTrait>(
    connection: &DatabaseConnection,
) -> Result<(), DatabaseError> {
    M::up(connection, None)
        .await
        .map_err(|err| DatabaseError::Migration(err.to_string()))
}
