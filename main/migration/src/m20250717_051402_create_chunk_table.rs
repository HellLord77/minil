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
                    .if_not_exists()
                    .col(pk_uuid(Chunk::Id))
                    .col(uuid_null(Chunk::ObjectId))
                    .col(uuid_null(Chunk::PartId))
                    .col(big_unsigned(Chunk::Index))
                    .col(binary(Chunk::Data))
                    .col(
                        timestamp_with_time_zone(Chunk::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chunk_object")
                            .from(Chunk::Table, Chunk::ObjectId)
                            .to(Object::Table, Object::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chunk_part")
                            .from(Chunk::Table, Chunk::PartId)
                            .to(Part::Table, Part::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::expr(
                            Expr::col(Chunk::ObjectId)
                                .is_not_null()
                                .and(Expr::col(Chunk::PartId).is_null()),
                        )
                        .or(Expr::col(Chunk::ObjectId)
                            .is_null()
                            .and(Expr::col(Chunk::PartId).is_not_null())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunk_object_id")
                    .table(Chunk::Table)
                    .col(Chunk::ObjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_chunk_part_id")
                    .table(Chunk::Table)
                    .col(Chunk::PartId)
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
                    .name("idx_chunk_object_id_part_id_index")
                    .table(Chunk::Table)
                    .col(Chunk::ObjectId)
                    .col(Chunk::PartId)
                    .col(Chunk::Index)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_chunk_object_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_chunk_part_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_chunk_index").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_chunk_object_id_part_id_index")
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
enum Object {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Part {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Chunk {
    Table,
    Id,
    ObjectId,
    PartId,
    Index,
    Data,
    CreatedAt,
}
