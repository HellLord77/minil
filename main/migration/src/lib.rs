pub use sea_orm_migration::prelude::*;

mod m20250616_000001_create_owner_table;
mod m20250617_000002_create_bucket_table;
mod m20250618_000003_create_object_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250616_000001_create_owner_table::Migration),
            Box::new(m20250617_000002_create_bucket_table::Migration),
            Box::new(m20250618_000003_create_object_table::Migration),
        ]
    }
}
