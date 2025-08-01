use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "bucket")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed, unique)]
    pub owner_id: Uuid,

    #[sea_orm(indexed, unique)]
    pub name: String,

    pub mfa_delete: Option<bool>,

    pub versioning: Option<bool>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    pub updated_at: Option<DateTimeUtc>,
}

impl Model {
    pub fn last_modified(&self) -> DateTimeUtc {
        self.updated_at.unwrap_or(self.created_at)
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Owner",
        from = "Column::OwnerId",
        to = "super::owner::Column::Id"
    )]
    Owner,

    #[sea_orm(has_many = "Upload")]
    Upload,

    #[sea_orm(has_many = "Object")]
    Object,
}

impl Related<Owner> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<Upload> for Entity {
    fn to() -> RelationDef {
        Relation::Upload.def()
    }
}

impl Related<Object> for Entity {
    fn to() -> RelationDef {
        Relation::Object.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
