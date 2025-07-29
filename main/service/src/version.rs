use futures::Stream;
use futures::StreamExt;
use mime::Mime;
use minil_entity::object;
use minil_entity::prelude::*;
use minil_entity::version;
use sea_orm::prelude::*;
use sea_orm::*;
use tokio::io::AsyncRead;

use crate::InsRes;
use crate::PartMutation;
use crate::error::DbRes;
use crate::utils::SelectExt;

pub struct VersionQuery;

impl VersionQuery {
    pub async fn find_by_object_id(
        db: &(impl ConnectionTrait + StreamTrait),
        object_id: Uuid,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<version::Model>>> {
        Version::find()
            .filter(version::Column::ObjectId.eq(object_id))
            .order_by_desc(version::Column::CreatedAt)
            .limit(limit)
            .stream(db)
            .await
    }

    pub async fn find_object_by_bucket_id(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        prefix: Option<&str>,
        continuation_token: Option<(&str, Option<&str>)>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<(version::Model, object::Model)>>> {
        let mut query = Version::find()
            .find_related(Object)
            .filter(object::Column::BucketId.eq(bucket_id));
        if let Some(prefix) = prefix {
            query = query.filter(object::Column::Key.starts_with(prefix));
        }
        if let Some((key_marker, id_marker)) = continuation_token {
            query = match id_marker {
                Some(id_marker) => query
                    .filter(object::Column::Key.gte(key_marker))
                    .filter(version::Column::Id.gt(id_marker)),
                None => query.filter(object::Column::Key.gt(key_marker)),
            };
        }
        Ok(query
            .order_by_asc(object::Column::Key)
            .order_by_desc(version::Column::CreatedAt)
            .limit(limit)
            .stream(db)
            .await?
            .map(|res| {
                res.map(|(version, object)| (version, object.unwrap_or_else(|| unreachable!())))
            }))
    }
}

pub struct VersionMutation;

impl VersionMutation {
    pub async fn insert(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
        object_id: Uuid,
        mime: Mime,
        read: impl Unpin + AsyncRead,
    ) -> InsRes<version::Model> {
        let part = PartMutation::insert(db, None, Some(id), 1, read).await?;

        let version = version::ActiveModel {
            id: Set(id),
            object_id: Set(object_id),
            mime: Set(mime.to_string()),
            size: Set(part.size),
            crc32: Set(part.crc32),
            crc32c: Set(part.crc32c),
            crc64nvme: Set(part.crc64nvme),
            sha1: Set(part.sha1),
            sha256: Set(part.sha256),
            md5: Set(part.md5),
            e_tag: Set(part.e_tag),
            ..Default::default()
        };

        Ok(Version::insert(version).exec_with_returning(db).await?)
    }

    pub async fn delete_by_object_id(
        db: &impl ConnectionTrait,
        object_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Version::delete_many()
            .filter(version::Column::ObjectId.eq(object_id))
            .exec(db)
            .await
    }
}
