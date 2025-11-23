use sea_orm_migration::prelude::*;

use crate::m20251118_122741_create_device_table::Device;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Measurement::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Measurement::Time)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Measurement::DeviceId).integer().not_null())
                    .col(ColumnDef::new(Measurement::Value).double().not_null())
                    .primary_key(
                        Index::create()
                            .col(Measurement::Time)
                            .col(Measurement::DeviceId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-measurement-device_id")
                            .from(Measurement::Table, Measurement::DeviceId)
                            .to(Device::Table, Device::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Measurement::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Measurement {
    Table,
    Time,
    DeviceId,
    Value,
}
