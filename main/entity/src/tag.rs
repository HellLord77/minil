use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed, unique)]
    pub tag_set_id: Uuid,

    #[sea_orm(indexed, unique)]
    pub key: String,

    pub value: String,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "TagSet",
        from = "Column::TagSetId",
        to = "super::tag_set::Column::Id"
    )]
    TagSet,
}

impl Related<TagSet> for Entity {
    fn to() -> RelationDef {
        Relation::TagSet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
