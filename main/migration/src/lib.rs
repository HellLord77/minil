pub use sea_orm_migration::prelude::*;

mod m20250616_055403_create_owner_table;
mod m20250617_042714_create_bucket_table;
mod m20250617_045245_create_upload_table;
mod m20250617_090716_create_upload_part_table;
mod m20250618_055951_create_object_table;
mod m20250622_144557_create_version_table;
mod m20250714_045800_create_version_part_table;
mod m20250717_051402_create_chunk_table;
mod m20250802_161509_create_tag_set_table;
mod m20250802_162159_create_tag_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250616_055403_create_owner_table::Migration),
            Box::new(m20250617_042714_create_bucket_table::Migration),
            Box::new(m20250617_045245_create_upload_table::Migration),
            Box::new(m20250617_090716_create_upload_part_table::Migration),
            Box::new(m20250618_055951_create_object_table::Migration),
            Box::new(m20250622_144557_create_version_table::Migration),
            Box::new(m20250714_045800_create_version_part_table::Migration),
            Box::new(m20250717_051402_create_chunk_table::Migration),
            Box::new(m20250802_161509_create_tag_set_table::Migration),
            Box::new(m20250802_162159_create_tag_table::Migration),
        ]
    }
}
