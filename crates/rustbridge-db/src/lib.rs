//! `SeaORM` integration for `MySQL` connections, transactions, and migrations.

mod connection;
mod error;
mod migration;
mod transaction;

pub use connection::{build_database_url, connect, ping};
pub use error::DatabaseError;
pub use migration::apply_migrations;
pub use transaction::run_in_transaction;

pub use sea_orm;
pub use sea_orm_migration;
