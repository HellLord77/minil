use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "bucket")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique, indexed)]
    pub owner_id: Uuid,

    #[sea_orm(unique, indexed)]
    pub name: String,

    pub region: String,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Owner",
        from = "Column::OwnerId",
        to = "super::owner::Column::Id"
    )]
    Owner,

    #[sea_orm(has_many = "Object")]
    Object,
}

impl Related<Owner> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
    }
}

impl Related<Object> for Entity {
    fn to() -> RelationDef {
        Relation::Object.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
