use sea_orm_migration::prelude::*;

use crate::m20251118_122736_create_user_table::User;
use crate::m20251118_122738_create_reservoir_table::Reservoir;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Device::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Device::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Device::UserId).integer().not_null())
                    .col(ColumnDef::new(Device::ReservoirId).integer().null())
                    .col(ColumnDef::new(Device::Name).string().not_null())
                    .col(
                        ColumnDef::new(Device::ApiKey)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Device::Status).string().default("offline"))
                    .col(
                        ColumnDef::new(Device::LastSeen)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-device-user_id")
                            .from(Device::Table, Device::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-device-reservoir_id")
                            .from(Device::Table, Device::ReservoirId)
                            .to(Reservoir::Table, Reservoir::Id)
                            .on_delete(ForeignKeyAction::SetNull) // delete reservoir without deleting device
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Device::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Device {
    Table,
    Id,
    UserId,
    ReservoirId,
    Name,
    ApiKey,
    Status,
    LastSeen,
}
