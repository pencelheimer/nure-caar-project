use sea_orm_migration::prelude::*;

use crate::m20251118_122736_create_user_table::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reservoir::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reservoir::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reservoir::UserId).integer().not_null())
                    .col(ColumnDef::new(Reservoir::Name).string().not_null())
                    .col(ColumnDef::new(Reservoir::Description).text().null())
                    .col(ColumnDef::new(Reservoir::Capacity).double().not_null())
                    .col(ColumnDef::new(Reservoir::Location).string().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-reservoir-user_id")
                            .from(Reservoir::Table, Reservoir::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reservoir::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Reservoir {
    Table,
    Id,
    UserId,
    Name,
    Description,
    Capacity,
    Location,
}
