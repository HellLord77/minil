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
                    .table(Part::Table)
                    .col(pk_uuid(Part::Id))
                    .col(uuid(Part::UploadId))
                    .col(small_unsigned(Part::Number))
                    .col(string(Part::Mime))
                    .col(big_unsigned(Part::Size))
                    .col(binary_len(Part::Crc32, 4))
                    .col(binary_len(Part::Crc32c, 4))
                    .col(binary_len(Part::Crc64nvme, 8))
                    .col(binary_len(Part::Sha1, 20))
                    .col(binary_len(Part::Sha256, 32))
                    .col(binary_len(Part::Md5, 16))
                    .col(
                        timestamp_with_time_zone(Part::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Part::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_part_upload")
                            .from(Part::Table, Part::UploadId)
                            .to(Upload::Table, Upload::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_part_upload_id")
                    .table(Part::Table)
                    .col(Part::UploadId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_part_number")
                    .table(Part::Table)
                    .col(Part::Number)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_part_upload_id_number")
                    .table(Part::Table)
                    .col(Part::UploadId)
                    .col(Part::Number)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_part_upload_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_part_number").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_part_upload_id_number").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Part::Table).to_owned())
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
enum Part {
    Table,
    Id,
    UploadId,
    Number,
    Mime,
    Size,
    Crc32,
    Crc32c,
    Crc64nvme,
    Sha1,
    Sha256,
    Md5,
    CreatedAt,
    UpdatedAt,
}
