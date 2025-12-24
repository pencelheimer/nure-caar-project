use crate::{
    error::AppError, //
    models::entities::{
        audit_log, //
        prelude::AuditLog,
    },
    views::admin::LogQuery,
};

use sea_orm::*;

pub struct AuditLogs;

impl AuditLogs {
    pub async fn find_filtered(
        db: &DatabaseConnection,
        query: LogQuery,
    ) -> Result<Vec<audit_log::Model>, AppError> {
        let mut select = AuditLog::find().order_by_desc(audit_log::Column::ChangedAt);

        if let Some(table_name) = query.table_name {
            select = select.filter(audit_log::Column::TableName.eq(table_name));
        }

        if let Some(record_id) = query.record_id {
            select = select.filter(audit_log::Column::RecordId.eq(record_id));
        }

        if let Some(operation) = query.operation {
            select = select.filter(audit_log::Column::Operation.eq(operation));
        }

        if let Some(limit) = query.limit {
            select = select.limit(limit);
        }

        if let Some(offset) = query.offset {
            select = select.offset(offset);
        }

        Ok(select.all(db).await?)
    }
}
