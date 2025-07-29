use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use mime::Mime;
use minil_entity::bucket;
use minil_entity::object;
use minil_entity::prelude::*;
use minil_entity::version;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_query::*;
use tokio::io::AsyncRead;

use crate::InsRes;
use crate::PartMutation;
use crate::error::DbRes;
use crate::utils::ExprExt;
use crate::utils::SelectExt;
use crate::utils::UpdateManyExt;

pub struct VersionQuery;

impl VersionQuery {
    pub async fn find(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
    ) -> DbRes<Option<version::Model>> {
        Version::find_by_id(id).one(db).await
    }

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
        let part = PartMutation::insert(db, None, Some(id), 1, Some(0), read).await?;

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

        Ok(Version::insert(version)
            .on_conflict(
                OnConflict::column(version::Column::Id)
                    .update_columns([
                        version::Column::Mime,
                        version::Column::Size,
                        version::Column::Crc32,
                        version::Column::Crc32c,
                        version::Column::Crc64nvme,
                        version::Column::Sha1,
                        version::Column::Sha256,
                        version::Column::Md5,
                        version::Column::ETag,
                    ])
                    .value(version::Column::UpdatedAt, Expr::current_timestamp())
                    .value(version::Column::DeletedAt, Expr::null())
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await?)
    }

    pub async fn update_deleted_at(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
    ) -> DbRes<Option<version::Model>> {
        // fixme PartMutation::delete_by_version_id(db, id).await?;

        Version::update_many()
            .filter(version::Column::Id.eq(id))
            .filter(version::Column::DeletedAt.is_null())
            .col_expr(bucket::Column::UpdatedAt, Expr::current_timestamp().into())
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }

    #[deprecated]
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
