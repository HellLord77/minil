use futures::TryStreamExt;
use minil_entity::prelude::*;
use minil_entity::tag_set;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_ext::prelude::*;
use sea_query::*;

use crate::TagMutation;
use crate::error::DbRes;

pub struct TagSetQuery;

impl TagSetQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        bucket_id: Option<Uuid>,
        upload_id: Option<Uuid>,
        version_id: Option<Uuid>,
    ) -> DbRes<Option<tag_set::Model>> {
        TagSet::find()
            .filter(tag_set::Column::BucketId.eq(bucket_id))
            .filter(tag_set::Column::UploadId.eq(upload_id))
            .filter(tag_set::Column::VersionId.eq(version_id))
            .one(db)
            .await
    }
}

pub struct TagSetMutation;

impl TagSetMutation {
    async fn upsert(
        db: &impl ConnectionTrait,
        id: Uuid,
        bucket_id: Option<Uuid>,
        upload_id: Option<Uuid>,
        version_id: Option<Uuid>,
    ) -> DbRes<tag_set::Model> {
        let tag_set = tag_set::ActiveModel {
            id: Set(id),
            bucket_id: Set(bucket_id),
            upload_id: Set(upload_id),
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

    pub async fn upsert_with_tag(
        db: &impl ConnectionTrait,
        bucket_id: Option<Uuid>,
        upload_id: Option<Uuid>,
        version_id: Option<Uuid>,
        iter: impl Iterator<Item = (String, String)>,
    ) -> DbRes<tag_set::Model> {
        let id =
            if let Some(tag_set) = TagSetQuery::find(db, bucket_id, upload_id, version_id).await? {
                TagMutation::delete_many(db, tag_set.id).await?;

                tag_set.id
            } else {
                Uuid::new_v4()
            };

        let tag_set = TagSetMutation::upsert(db, id, bucket_id, upload_id, version_id).await?;
        TagMutation::insert_many(db, tag_set.id, iter).await?;

        Ok(tag_set)
    }

    pub async fn delete(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Option<Uuid>,
        upload_id: Option<Uuid>,
        version_id: Option<Uuid>,
    ) -> DbRes<Option<tag_set::Model>> {
        TagSet::delete_many()
            .filter(tag_set::Column::BucketId.eq(bucket_id))
            .filter(tag_set::Column::UploadId.eq(upload_id))
            .filter(tag_set::Column::VersionId.eq(version_id))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }
}
