use minil_entity::prelude::*;
use minil_entity::tag;
use sea_orm::prelude::*;
use sea_orm::*;

use crate::error::DbRes;

pub struct TagQuery;

impl TagQuery {}

pub struct TagMutation;

impl TagMutation {
    pub(super) async fn insert_many(
        db: &impl ConnectionTrait,
        tag_set_id: Uuid,
        iter: impl Iterator<Item = (String, String)>,
    ) -> DbRes<InsertResult<tag::ActiveModel>> {
        let tags = iter.map(|(key, value)| tag::ActiveModel {
            id: Set(Uuid::new_v4()),
            tag_set_id: Set(tag_set_id),
            key: Set(key),
            value: Set(value),
            ..Default::default()
        });

        Tag::insert_many(tags).exec(db).await
    }

    pub(super) async fn delete_by_tag_set_id(
        db: &impl ConnectionTrait,
        tag_set_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Tag::delete_many()
            .filter(tag::Column::TagSetId.eq(tag_set_id))
            .exec(db)
            .await
    }
}
