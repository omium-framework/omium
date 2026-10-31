use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Examples::Table)
                    .add_column(ColumnDef::new(Examples::Description).string_len(512).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Examples::Table)
                    .drop_column(Examples::Description)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Examples {
    Table,
    Description,
}
