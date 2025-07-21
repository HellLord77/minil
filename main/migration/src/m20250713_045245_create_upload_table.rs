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
                    .table(Upload::Table)
                    .col(pk_uuid(Upload::Id))
                    .col(uuid(Upload::BucketId))
                    .col(string(Upload::Key))
                    .col(string_null(Upload::Mime))
                    .col(
                        timestamp_with_time_zone(Upload::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_upload_bucket")
                            .from(Upload::Table, Upload::BucketId)
                            .to(Bucket::Table, Bucket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_upload_bucket_id")
                    .table(Upload::Table)
                    .col(Upload::BucketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_upload_key")
                    .table(Upload::Table)
                    .col(Upload::Key)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_upload_bucket_id_key")
                    .table(Upload::Table)
                    .col(Upload::BucketId)
                    .col(Upload::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_upload_bucket_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_upload_key").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_upload_bucket_id_key").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Upload::Table).to_owned())
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
enum Upload {
    Table,
    Id,
    BucketId,
    Key,
    Mime,
    CreatedAt,
}
