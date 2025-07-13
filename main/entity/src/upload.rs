use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "upload")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique, indexed)]
    pub bucket_id: Uuid,

    #[sea_orm(unique, indexed)]
    pub key: String,

    pub mime: Option<String>,

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

    #[sea_orm(has_many = "Part")]
    Part,
}

impl Related<Bucket> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl Related<Part> for Entity {
    fn to() -> RelationDef {
        Relation::Part.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
