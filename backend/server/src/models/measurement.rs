use crate::{
    error::AppError, //
    models::entities::{
        measurement, //
        prelude::Measurement,
    },
    views::measurement::MeasurementHistoryQuery,
};

use chrono::{
    DateTime, //
    FixedOffset,
    Utc,
};

use sea_orm::*;

pub struct Measurements;

impl Measurements {
    pub async fn add(
        db: &DatabaseConnection,
        device_id: i32,
        value: f64,
        timestamp: Option<DateTime<FixedOffset>>,
    ) -> Result<measurement::Model, AppError> {
        let time = timestamp.unwrap_or_else(|| Utc::now().into());

        let active_model = measurement::ActiveModel {
            time: Set(time),
            device_id: Set(device_id),
            value: Set(value),
            ..Default::default()
        };

        let res = active_model.insert(db).await?;
        Ok(res)
    }

    pub async fn find_history(
        db: &DatabaseConnection,
        device_id: i32,
        query: MeasurementHistoryQuery,
    ) -> Result<Vec<measurement::Model>, AppError> {
        let mut select = Measurement::find()
            .filter(measurement::Column::DeviceId.eq(device_id))
            .order_by_desc(measurement::Column::Time);

        if let Some(from) = query.from {
            select = select.filter(measurement::Column::Time.gte(from));
        }

        if let Some(to) = query.to {
            select = select.filter(measurement::Column::Time.lte(to));
        }

        let limit = query.limit.unwrap_or(100);
        select = select.limit(limit);

        let data = select.all(db).await?;
        Ok(data)
    }
}
