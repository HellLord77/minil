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
                    .table(TagSet::Table)
                    .col(pk_uuid(TagSet::Id))
                    .col(uuid_null(TagSet::BucketId))
                    .col(uuid_null(TagSet::VersionId))
                    .col(
                        timestamp_with_time_zone(TagSet::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_with_time_zone_null(TagSet::UpdatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tag_set_bucket")
                            .from(TagSet::Table, TagSet::BucketId)
                            .to(Bucket::Table, Bucket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tag_set_version")
                            .from(TagSet::Table, TagSet::VersionId)
                            .to(Version::Table, Version::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .check(
                        Expr::expr(
                            Expr::col(TagSet::BucketId)
                                .is_not_null()
                                .and(Expr::col(TagSet::VersionId).is_null()),
                        )
                        .or(Expr::col(TagSet::BucketId)
                            .is_null()
                            .and(Expr::col(TagSet::VersionId).is_not_null())),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tag_set_bucket_id")
                    .table(TagSet::Table)
                    .col(TagSet::BucketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tag_set_version_id")
                    .table(TagSet::Table)
                    .col(TagSet::VersionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tag_set_bucket_id_version_id")
                    .table(TagSet::Table)
                    .col(TagSet::BucketId)
                    .col(TagSet::VersionId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_tag_set_bucket_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_tag_set_version_id").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_tag_set_bucket_id_version_id")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TagSet::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Bucket {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Version {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TagSet {
    Table,
    Id,
    BucketId,
    VersionId,
    CreatedAt,
    UpdatedAt,
}
