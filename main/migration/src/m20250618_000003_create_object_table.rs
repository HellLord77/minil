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
                    .table(Object::Table)
                    .if_not_exists()
                    .col(pk_uuid(Object::Id))
                    .col(uuid(Object::BucketId))
                    .col(string(Object::Key))
                    .col(big_unsigned(Object::Size))
                    .col(timestamp(Object::LastModified))
                    .col(string(Object::ETag))
                    .col(timestamp(Object::CreatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_object_bucket")
                            .from(Object::Table, Object::BucketId)
                            .to(Bucket::Table, Bucket::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_object_bucket_id")
                    .table(Object::Table)
                    .col(Object::BucketId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_object_key")
                    .table(Object::Table)
                    .col(Object::Key)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_object_bucket_id_key")
                    .table(Object::Table)
                    .col(Object::BucketId)
                    .col(Object::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_object_bucket_id_key").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_object_key").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_object_bucket_id").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Object::Table).to_owned())
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
enum Object {
    Table,
    Id,
    BucketId,
    Key,
    Size,
    LastModified,
    ETag,
    CreatedAt,
}
