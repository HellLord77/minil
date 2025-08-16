use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "chunk")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed, unique)]
    pub upload_part_id: Option<Uuid>,

    #[sea_orm(indexed, unique)]
    pub version_part_id: Option<Uuid>,

    #[sea_orm(indexed, unique)]
    pub index: i64,

    pub start: i64,

    pub end: i64,

    pub data: Vec<u8>,

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
        belongs_to = "UploadPart",
        from = "Column::UploadPartId",
        to = "super::upload_part::Column::Id"
    )]
    UploadPart,

    #[sea_orm(
        belongs_to = "VersionPart",
        from = "Column::VersionPartId",
        to = "super::version_part::Column::Id"
    )]
    VersionPart,
}

impl Related<UploadPart> for Entity {
    fn to() -> RelationDef {
        Relation::UploadPart.def()
    }
}

impl Related<VersionPart> for Entity {
    fn to() -> RelationDef {
        Relation::VersionPart.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
