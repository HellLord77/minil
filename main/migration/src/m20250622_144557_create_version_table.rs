use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Version::Table)
                    .col(pk_uuid(Version::Id))
                    .col(uuid(Version::ObjectId))
                    .col(boolean(Version::Versioning))
                    .col(small_integer_null(Version::PartsCount))
                    .col(string_null(Version::Mime))
                    .col(big_unsigned_null(Version::Size))
                    .col(binary_len_null(Version::Crc32, 4))
                    .col(binary_len_null(Version::Crc32c, 4))
                    .col(binary_len_null(Version::Crc64nvme, 8))
                    .col(binary_len_null(Version::Sha1, 20))
                    .col(binary_len_null(Version::Sha256, 32))
                    .col(binary_len_null(Version::Md5, 16))
                    .col(string_null(Version::ETag))
                    .col(
                        timestamp_with_time_zone(Version::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Version::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_version_object")
                            .from(Version::Table, Version::ObjectId)
                            .to(Object::Table, Object::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    // todo check option
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_version_object_id")
                    .table(Version::Table)
                    .col(Version::ObjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Object::Table)
                    .add_foreign_key(
                        ForeignKey::create()
                            .name("fk_object_version")
                            .from(Object::Table, Object::VersionId)
                            .to(Version::Table, Version::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .get_foreign_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(include_str!(
                "../sql/m20250622_144557_create_version_table.sql"
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Object::Table)
                    .drop_foreign_key("fk_object_version")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_version_object_id").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Version::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Object {
    Table,
    Id,
    VersionId,
}

#[derive(DeriveIden)]
enum Version {
    Table,
    Id,
    ObjectId,
    Versioning,
    PartsCount,
    Mime,
    Size,
    Crc32,
    Crc32c,
    Crc64nvme,
    Sha1,
    Sha256,
    Md5,
    ETag,
    CreatedAt,
    UpdatedAt,
}
