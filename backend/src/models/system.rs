use crate::{
    error::AppError,
    models::{
        entities::{
            alert_rule,
            prelude::{AlertRule, Device, Reservoir, User},
        },
        user,
    },
    views::admin::SystemStatsResponse,
};
use sea_orm::*;

pub struct System;

impl System {
    pub async fn get_stats(db: &DatabaseConnection) -> Result<SystemStatsResponse, AppError> {
        let (total_users, total_reservoirs, total_devices, alert_rules_active) = tokio::try_join!(
            User::find().count(db),
            Reservoir::find().count(db),
            Device::find().count(db),
            AlertRule::find()
                .filter(alert_rule::Column::IsActive.eq(true))
                .count(db)
        )?;

        Ok(SystemStatsResponse {
            total_users,
            total_reservoirs,
            total_devices,
            alert_rules_active,
        })
    }
}
