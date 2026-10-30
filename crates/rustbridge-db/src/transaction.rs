//! Transaction boundary helper for multi-step operations.

use std::future::Future;

use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};

/// Runs an async closure with an owned transaction handle.
///
/// The closure must call [`DatabaseTransaction::commit`] on success. Uncommitted
/// transactions roll back when dropped.
///
/// # Errors
///
/// Returns [`DbErr`] when begin, the closure, commit, or rollback fails.
pub async fn run_in_transaction<F, Fut, T>(
    connection: &DatabaseConnection,
    operation: F,
) -> Result<T, DbErr>
where
    F: FnOnce(DatabaseTransaction) -> Fut,
    Fut: Future<Output = Result<T, DbErr>>,
{
    let transaction = connection.begin().await?;
    operation(transaction).await
}
