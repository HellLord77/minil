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
                    .table(Chunk::Table)
                    .col(pk_uuid(Chunk::Id))
                    .col(uuid_null(Chunk::UploadPartId))
                    .col(uuid_null(Chunk::VersionPartId))
                    .col(big_unsigned(Chunk::Index))
                    .col(big_unsigned(Chunk::Start))
                    .col(big_unsigned(Chunk::End))
                    .col(binary(Chunk::Data))
                    .col(
                        timestamp_with_time_zone(Chunk::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(Chunk::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chunk_upload_part")
                            .from(Chunk::Table, Chunk::UploadPartId)
                            .to(UploadPart::Table, UploadPart::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chunk_version_part")
                            .from(Chunk::Table, Chunk::VersionPartId)
                            .to(VersionPart::Table, VersionPart::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::case(Expr::col(Chunk::UploadPartId).is_not_null(), 1)
                            .finally(0)
                            .add(
                                Expr::case(Expr::col(Chunk::VersionPartId).is_not_null(), 1)
                                    .finally(0),
                            )
                            .eq(1),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunk_upload_part_id")
                    .table(Chunk::Table)
                    .col(Chunk::UploadPartId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunk_version_part_id")
                    .table(Chunk::Table)
                    .col(Chunk::VersionPartId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunk_index")
                    .table(Chunk::Table)
                    .col(Chunk::Index)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunk_upload_part_id_version_part_id_index")
                    .table(Chunk::Table)
                    .col(Chunk::UploadPartId)
                    .col(Chunk::VersionPartId)
                    .col(Chunk::Index)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(include_str!(
                "../sql/m20250717_051402_create_chunk_table.sql"
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_chunk_upload_part_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_chunk_version_part_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_chunk_index").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_chunk_upload_part_id_version_part_id_index")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Chunk::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UploadPart {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum VersionPart {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Chunk {
    Table,
    Id,
    UploadPartId,
    VersionPartId,
    Index,
    Start,
    End,
    Data,
    CreatedAt,
    UpdatedAt,
}
