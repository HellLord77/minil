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
                    .table(UploadPart::Table)
                    .col(pk_uuid(UploadPart::Id))
                    .col(uuid(UploadPart::UploadId))
                    .col(small_unsigned(UploadPart::Number))
                    .col(big_unsigned(UploadPart::Size))
                    .col(binary_len(UploadPart::Crc32, 4))
                    .col(binary_len(UploadPart::Crc32C, 4))
                    .col(binary_len(UploadPart::Crc64Nvme, 8))
                    .col(binary_len(UploadPart::Sha1, 20))
                    .col(binary_len(UploadPart::Sha256, 32))
                    .col(binary_len(UploadPart::Md5, 16))
                    .col(
                        timestamp_with_time_zone(UploadPart::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(UploadPart::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_upload_part_upload")
                            .from(UploadPart::Table, UploadPart::UploadId)
                            .to(Upload::Table, Upload::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_upload_part_upload_id")
                    .table(UploadPart::Table)
                    .col(UploadPart::UploadId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_upload_part_number")
                    .table(UploadPart::Table)
                    .col(UploadPart::Number)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_upload_part_upload_id_number")
                    .table(UploadPart::Table)
                    .col(UploadPart::UploadId)
                    .col(UploadPart::Number)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_upload_part_upload_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_upload_part_number").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_upload_part_upload_id_number")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UploadPart::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Upload {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UploadPart {
    Table,
    Id,
    UploadId,
    Number,
    Size,
    Crc32,
    Crc32C,
    Crc64Nvme,
    Sha1,
    Sha256,
    Md5,
    CreatedAt,
    UpdatedAt,
}
