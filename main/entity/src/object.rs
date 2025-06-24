use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "object")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique, indexed)]
    pub bucket_id: Uuid,

    #[sea_orm(unique, indexed)]
    pub key: String,

    pub size: u64,

    pub last_modified: DateTimeUtc,

    pub e_tag: String,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Bucket",
        from = "Column::BucketId",
        to = "super::bucket::Column::Id"
    )]
    Bucket,
}

impl Related<Bucket> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
