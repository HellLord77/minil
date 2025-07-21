use sea_orm::entity::prelude::*;

use super::prelude::*;

#[derive(Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "part")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(unique, indexed)]
    pub upload_id: Uuid,

    #[sea_orm(unique, indexed)]
    pub number: i16,

    pub size: i64,

    #[sea_orm(column_type = "Binary(4)")]
    pub crc32: Vec<u8>,

    #[sea_orm(column_type = "Binary(4)")]
    pub crc32c: Vec<u8>,

    #[sea_orm(column_type = "Binary(8)")]
    pub crc64nvme: Vec<u8>,

    #[sea_orm(column_type = "Binary(20)")]
    pub sha1: Vec<u8>,

    #[sea_orm(column_type = "Binary(32)")]
    pub sha256: Vec<u8>,

    #[sea_orm(column_type = "Binary(16)")]
    pub md5: Vec<u8>,

    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,

    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Upload",
        from = "Column::UploadId",
        to = "super::upload::Column::Id"
    )]
    Upload,
}

impl Related<Upload> for Entity {
    fn to() -> RelationDef {
        Relation::Upload.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
