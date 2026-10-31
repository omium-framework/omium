//! Minimal migrations for `omium-db` integration tests (no dependency on `omium-db`).

pub use sea_orm_migration::prelude::*;

mod m20250330_000001_create_examples_table;
mod m20250330_000002_add_examples_description;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250330_000001_create_examples_table::Migration),
            Box::new(m20250330_000002_add_examples_description::Migration),
        ]
    }
}
