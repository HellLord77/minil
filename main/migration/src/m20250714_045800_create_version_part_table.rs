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
                    .table(VersionPart::Table)
                    .col(pk_uuid(VersionPart::Id))
                    .col(uuid(VersionPart::VersionId))
                    .col(small_unsigned(VersionPart::Number))
                    .col(big_unsigned(VersionPart::Start))
                    .col(big_unsigned(VersionPart::End))
                    .col(big_unsigned(VersionPart::Size))
                    .col(binary_len(VersionPart::Crc32, 4))
                    .col(binary_len(VersionPart::Crc32C, 4))
                    .col(binary_len(VersionPart::Crc64Nvme, 8))
                    .col(binary_len(VersionPart::Sha1, 20))
                    .col(binary_len(VersionPart::Sha256, 32))
                    .col(binary_len(VersionPart::Md5, 16))
                    .col(
                        timestamp_with_time_zone(VersionPart::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_version_part_version")
                            .from(VersionPart::Table, VersionPart::VersionId)
                            .to(Version::Table, Version::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_version_part_version_id")
                    .table(VersionPart::Table)
                    .col(VersionPart::VersionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_version_part_number")
                    .table(VersionPart::Table)
                    .col(VersionPart::Number)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_version_part_version_id_number")
                    .table(VersionPart::Table)
                    .col(VersionPart::VersionId)
                    .col(VersionPart::Number)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(include_str!(
                "../sql/m20250714_045800_create_version_part_table.sql"
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_version_part_version_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_version_part_number").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_version_part_version_id_number")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(VersionPart::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Version {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum VersionPart {
    Table,
    Id,
    VersionId,
    Number,
    Start,
    End,
    Size,
    Crc32,
    Crc32C,
    Crc64Nvme,
    Sha1,
    Sha256,
    Md5,
    CreatedAt,
}
