use futures::TryStreamExt;
use minil_entity::prelude::*;
use minil_entity::tag_set;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_query::*;

use crate::TagMutation;
use crate::error::DbRes;
use crate::utils::DeleteManyExt;

pub struct TagSetQuery;

impl TagSetQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        bucket_id: Option<Uuid>,
        version_id: Option<Uuid>,
    ) -> DbRes<Option<tag_set::Model>> {
        let mut query = tag_set::Entity::find();
        if let Some(bucket_id) = bucket_id {
            query = query.filter(tag_set::Column::BucketId.eq(bucket_id));
        }
        if let Some(version_id) = version_id {
            query = query.filter(tag_set::Column::VersionId.eq(version_id));
        }
        query.one(db).await
    }
}

pub struct TagSetMutation;

impl TagSetMutation {
    pub async fn upsert_with_tag(
        db: &impl ConnectionTrait,
        bucket_id: Option<Uuid>,
        version_id: Option<Uuid>,
        iter: impl Iterator<Item = (String, String)>,
    ) -> DbRes<tag_set::Model> {
        let id = TagSetQuery::find(db, bucket_id, version_id)
            .await?
            .map_or_else(Uuid::new_v4, |model| model.id);
        TagMutation::delete_by_tag_set_id(db, id).await?;

        TagMutation::insert_many(db, id, iter).await?;

        let tag_set = tag_set::ActiveModel {
            id: Set(id),
            bucket_id: Set(bucket_id),
            version_id: Set(version_id),
            ..Default::default()
        };

        TagSet::insert(tag_set)
            .on_conflict(
                OnConflict::column(tag_set::Column::Id)
                    .value(tag_set::Column::UpdatedAt, Expr::current_timestamp())
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
    }

    pub async fn delete(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
    ) -> DbRes<Option<tag_set::Model>> {
        tag_set::Entity::delete_many()
            .filter(tag_set::Column::Id.eq(id))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }
}
