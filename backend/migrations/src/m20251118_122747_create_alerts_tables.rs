use sea_orm_migration::prelude::*;

use crate::m20251118_122738_create_reservoir_table::Reservoir;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlertRule::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AlertRule::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AlertRule::ReservoirId).integer().not_null())
                    .col(ColumnDef::new(AlertRule::ConditionType).string().not_null())
                    .col(ColumnDef::new(AlertRule::Threshold).double().not_null())
                    .col(ColumnDef::new(AlertRule::IsActive).boolean().default(true))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-alert_rule-reservoir_id")
                            .from(AlertRule::Table, AlertRule::ReservoirId)
                            .to(Reservoir::Table, Reservoir::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Alert::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alert::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Alert::RuleId).integer().not_null())
                    .col(
                        ColumnDef::new(Alert::TriggeredAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Alert::SentTo).string().not_null())
                    .col(ColumnDef::new(Alert::Status).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-alert-rule_id")
                            .from(Alert::Table, Alert::RuleId)
                            .to(AlertRule::Table, AlertRule::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alert::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AlertRule::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AlertRule {
    Table,
    Id,
    ReservoirId,
    ConditionType,
    Threshold,
    IsActive,
}

#[derive(Iden)]
enum Alert {
    Table,
    Id,
    RuleId,
    TriggeredAt,
    SentTo,
    Status,
}
