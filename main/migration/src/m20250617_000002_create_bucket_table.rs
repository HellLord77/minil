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
                    .table(Bucket::Table)
                    .if_not_exists()
                    .col(pk_uuid(Bucket::Id))
                    .col(uuid(Bucket::OwnerId))
                    .col(string(Bucket::Name))
                    .col(string(Bucket::Region))
                    .col(
                        timestamp_with_time_zone(Bucket::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bucket_owner")
                            .from(Bucket::Table, Bucket::OwnerId)
                            .to(Owner::Table, Owner::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bucket_owner_id")
                    .table(Bucket::Table)
                    .col(Bucket::OwnerId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bucket_name")
                    .table(Bucket::Table)
                    .col(Bucket::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bucket_owner_id_name")
                    .table(Bucket::Table)
                    .col(Bucket::OwnerId)
                    .col(Bucket::Name)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_bucket_owner_id_name").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_bucket_name").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_bucket_owner_id").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Bucket::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Owner {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Bucket {
    Table,
    Id,
    OwnerId,
    Name,
    Region,
    CreatedAt,
}
