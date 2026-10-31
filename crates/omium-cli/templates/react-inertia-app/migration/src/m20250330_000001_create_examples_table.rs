use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Examples::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Examples::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Examples::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Examples::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Examples::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Examples {
    Table,
    Id,
    Name,
    CreatedAt,
}
