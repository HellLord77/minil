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

    pub parts_count: Option<i16>,

    pub mime: Option<String>,

    pub size: Option<i64>,

    #[sea_orm(column_type = "Binary(4)")]
    pub crc32: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(4)")]
    pub crc32_c: Option<Vec<u8>>,

    #[sea_orm(column_type = "Binary(8)")]
    pub crc64_nvme: Option<Vec<u8>>,

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

impl Model {
    #[must_use]
    pub fn id(&self) -> Uuid {
        if self.versioning {
            self.id
        } else {
            Uuid::nil()
        }
    }

    #[must_use]
    pub fn mp_parts_count(&self) -> Option<u16> {
        let parts_count = self.parts_count.unwrap() as u16;
        (parts_count != 0).then_some(parts_count)
    }

    #[must_use]
    pub fn size(&self) -> u64 {
        self.size.unwrap() as u64
    }

    #[must_use]
    pub fn e_tag(&self) -> String {
        self.e_tag
            .clone()
            .unwrap_or_else(|| format!("\"{}\"", hex::encode(self.md5.as_ref().unwrap())))
    }

    #[must_use]
    pub fn last_modified(&self) -> DateTimeUtc {
        self.updated_at.unwrap_or(self.created_at)
    }
}

#[derive(Debug, Clone, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Object",
        from = "Column::ObjectId",
        to = "super::object::Column::Id"
    )]
    Object,

    #[sea_orm(has_many = "VersionPart")]
    VersionPart,

    #[sea_orm(has_one = "TagSet")]
    TagSet,
}

impl Related<Object> for Entity {
    fn to() -> RelationDef {
        Relation::Object.def()
    }
}

impl Related<VersionPart> for Entity {
    fn to() -> RelationDef {
        Relation::VersionPart.def()
    }
}

impl Related<TagSet> for Entity {
    fn to() -> RelationDef {
        Relation::TagSet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
