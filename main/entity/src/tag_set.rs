use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "tag_set")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed, unique)]
    pub bucket_id: Option<Uuid>,

    #[sea_orm(indexed, unique)]
    pub upload_id: Option<Uuid>,

    #[sea_orm(indexed, unique)]
    pub version_id: Option<Uuid>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    pub updated_at: Option<DateTimeUtc>,
}

impl Model {
    #[must_use]
    pub fn last_modified(&self) -> DateTimeUtc {
        self.updated_at.unwrap_or(self.created_at)
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Bucket",
        from = "Column::BucketId",
        to = "super::bucket::Column::Id"
    )]
    Bucket,

    #[sea_orm(
        belongs_to = "Upload",
        from = "Column::UploadId",
        to = "super::upload::Column::Id"
    )]
    Upload,

    #[sea_orm(
        belongs_to = "Version",
        from = "Column::VersionId",
        to = "super::version::Column::Id"
    )]
    Version,

    #[sea_orm(has_many = "Tag")]
    Tag,
}

impl Related<Bucket> for Entity {
    fn to() -> RelationDef {
        Relation::Bucket.def()
    }
}

impl Related<Upload> for Entity {
    fn to() -> RelationDef {
        Relation::Upload.def()
    }
}

impl Related<Version> for Entity {
    fn to() -> RelationDef {
        Relation::Version.def()
    }
}

impl Related<Tag> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
