use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "version")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(indexed)]
    pub object_id: Uuid,

    pub versioning: bool,

    pub part_count: Option<i16>,

    pub mime: Option<String>,

    pub size: Option<i64>,

    #[sea_orm(column_type = "Binary(4)")]
    pub crc32: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(4)")]
    pub crc32c: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(8)")]
    pub crc64nvme: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(20)")]
    pub sha1: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(32)")]
    pub sha256: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(16)")]
    pub md5: Option<Vec<u8>>,

    pub e_tag: Option<String>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Object",
        from = "Column::ObjectId",
        to = "super::object::Column::Id"
    )]
    Object,

    #[sea_orm(has_many = "Part")]
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
