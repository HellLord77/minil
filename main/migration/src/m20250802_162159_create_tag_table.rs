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
                    .table(Tag::Table)
                    .col(pk_uuid(Tag::Id))
                    .col(uuid(Tag::TagSetId))
                    .col(string(Tag::Key))
                    .col(string(Tag::Value))
                    .col(
                        timestamp_with_time_zone(Tag::CreatedAt).default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tag_tag_set")
                            .from(Tag::Table, Tag::TagSetId)
                            .to(TagSet::Table, TagSet::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tag_tag_set_id")
                    .table(Tag::Table)
                    .col(Tag::TagSetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tag_tag_set_id_key")
                    .table(Tag::Table)
                    .col(Tag::TagSetId)
                    .col(Tag::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(include_str!("../sql/m20250802_162159_create_tag_table.sql"))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("idx_tag_tag_set_id").to_owned())
            .await?;

        manager
            .drop_index(Index::drop().name("idx_tag_tag_set_id_key").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum TagSet {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    #[allow(clippy::enum_variant_names)]
    TagSetId,
    Key,
    Value,
    CreatedAt,
}
