use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "object")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed, unique)]
    pub bucket_id: Uuid,

    #[sea_orm(indexed, unique)]
    pub key: String,

    #[sea_orm(indexed, unique)]
    pub version_id: Uuid,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Bucket",
        from = "Column::BucketId",
        to = "super::bucket::Column::Id"
    )]
    Bucket,

    #[sea_orm(has_many = "Version")]
    Version,

    #[sea_orm(
        has_one = "Version",
        from = "Column::VersionId",
        to = "super::version::Column::Id"
    )]
    LatestVersion,
}

impl Related<Bucket> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl Related<Version> for Entity {
    fn to() -> RelationDef {
        Relation::Version.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
