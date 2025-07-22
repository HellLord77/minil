use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "chunk")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique, indexed)]
    pub object_id: Option<Uuid>,

    #[sea_orm(unique, indexed)]
    pub part_id: Option<Uuid>,

    #[sea_orm(unique, indexed)]
    pub index: i64,

    pub start: i64,

    pub end: i64,

    pub data: Vec<u8>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Object",
        from = "Column::ObjectId",
        to = "super::object::Column::Id"
    )]
    Object,

    #[sea_orm(
        belongs_to = "Part",
        from = "Column::PartId",
        to = "super::part::Column::Id"
    )]
    Part,
}

impl Related<Object> for Entity {
    fn to() -> RelationDef {
        Relation::Object.def()
    }
}

impl Related<Part> for Entity {
    fn to() -> RelationDef {
        Relation::Part.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
