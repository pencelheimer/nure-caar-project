pub use sea_orm_migration::prelude::*;

mod m20251118_122736_create_user_table;
mod m20251118_122738_create_reservoir_table;
mod m20251118_122741_create_device_table;
mod m20251118_122744_create_measurement_table;
mod m20251118_122747_create_alerts_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251118_122736_create_user_table::Migration),
            Box::new(m20251118_122738_create_reservoir_table::Migration),
            Box::new(m20251118_122741_create_device_table::Migration),
            Box::new(m20251118_122744_create_measurement_table::Migration),
            Box::new(m20251118_122747_create_alerts_tables::Migration),
        ]
    }
}
