use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "upload")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub bucket_id: Uuid,

    #[sea_orm(indexed)]
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

    #[sea_orm(has_many = "UploadPart")]
    UploadPart,

    #[sea_orm(has_one = "TagSet")]
    TagSet,
}

impl Related<Bucket> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl Related<UploadPart> for Entity {
    fn to() -> RelationDef {
        Relation::UploadPart.def()
    }
}

impl Related<TagSet> for Entity {
    fn to() -> RelationDef {
        Relation::TagSet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
