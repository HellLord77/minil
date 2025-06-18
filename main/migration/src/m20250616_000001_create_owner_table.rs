use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Owner::Table)
                    .if_not_exists()
                    .col(pk_uuid(Owner::Id))
                    .col(string_uniq(Owner::Name))
                    .col(timestamp(Owner::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_owner_name")
                    .table(Owner::Table)
                    .col(Owner::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Owner::Table)
                    .columns([Owner::Id, Owner::Name])
                    .values_panic([Uuid::new_v4().into(), "minil".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::delete()
                    .from_table(Owner::Table)
                    .and_where(Expr::col(Owner::Name).eq("minil"))
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(Index::drop().name("idx_owner_name").to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Owner::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Owner {
    Table,
    Id,
    Name,
    CreatedAt,
}
