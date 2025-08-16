use futures::Stream;
use futures::TryStreamExt;
use mime::Mime;
use minil_entity::prelude::*;
use minil_entity::upload;
use sea_orm::Set;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_ext::prelude::*;
use uuid::Uuid;

use crate::error::DbRes;

pub struct UploadQuery;

impl UploadQuery {
    pub async fn find_many(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        prefix: Option<&str>,
        key_marker: Option<&str>,
        upload_id_marker: Option<&str>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<upload::Model>>> {
        Upload::find()
            .filter(upload::Column::BucketId.eq(bucket_id))
            .apply_if(prefix, |query, prefix| {
                query.filter(upload::Column::Key.eq(prefix))
            })
            .apply_if(key_marker, |query, key_marker| {
                if upload_id_marker.is_some() {
                    query.filter(upload::Column::Key.gte(key_marker))
                } else {
                    query.filter(upload::Column::Key.gt(key_marker))
                }
            })
            .apply_if(upload_id_marker, |query, upload_id_marker| {
                query.filter(upload::Column::Id.gt(upload_id_marker))
            })
            .order_by_asc(upload::Column::Key)
            .order_by_asc(upload::Column::Id)
            .limit(limit)
            .stream(db)
            .await
    }
}

pub struct UploadMutation;

impl UploadMutation {
    pub async fn insert(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: String,
        mime: Option<&Mime>,
    ) -> DbRes<upload::Model> {
        let upload = upload::ActiveModel {
            id: Set(Uuid::new_v4()),
            bucket_id: Set(bucket_id),
            key: Set(key),
            mime: Set(mime.map(ToString::to_string)),
            ..Default::default()
        };

        Upload::insert(upload).exec_with_returning(db).await
    }

    pub async fn delete(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<upload::Model>> {
        Upload::delete_many()
            .filter(upload::Column::Id.eq(id))
            .filter(upload::Column::BucketId.eq(bucket_id))
            .filter(upload::Column::Key.eq(key))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }
}
