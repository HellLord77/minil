use futures::Stream;
use futures::TryStreamExt;
use mime::Mime;
use minil_entity::object;
use minil_entity::prelude::*;
use minil_entity::version;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_ext::prelude::*;
use sea_query::*;
use tokio::io::AsyncRead;

use crate::InsRes;
use crate::PartMutation;
use crate::error::DbRes;

pub struct VersionQuery;

impl VersionQuery {
    pub async fn find_2nd_latest(
        db: &impl ConnectionTrait,
        object_id: Uuid,
    ) -> DbRes<Option<version::Model>> {
        Version::find()
            .filter(version::Column::ObjectId.eq(object_id))
            .order_by_desc(version::Column::CreatedAt)
            .offset(Some(1))
            .one(db)
            .await
    }

    pub async fn find_many_both_object(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        prefix: Option<&str>,
        key_maker: Option<&str>,
        version_id_marker: Option<&str>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<(version::Model, object::Model)>>> {
        let mut query = Version::find()
            .find_both_related(Object)
            .filter(object::Column::BucketId.eq(bucket_id));
        if let Some(prefix) = prefix {
            query = query.filter(object::Column::Key.starts_with(prefix));
        }
        if let Some(key_marker) = key_maker {
            query = query.filter(object::Column::Key.gte(key_marker));
        }
        if let Some(version_id_marker) = version_id_marker {
            query = query.filter(version::Column::Id.gte(version_id_marker));
        }
        query
            .order_by_asc(object::Column::Key)
            .order_by_desc(version::Column::CreatedAt)
            .limit(limit)
            .stream_both(db)
            .await
    }
}

pub struct VersionMutation;

impl VersionMutation {
    #[allow(dead_code)]
    #[deprecated]
    async fn insert_delete_marker(
        db: &impl ConnectionTrait,
        object_id: Uuid,
        versioning: bool,
    ) -> DbRes<version::Model> {
        let version = version::ActiveModel {
            id: Set(Uuid::new_v4()),
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(None),
            mime: Set(None),
            size: Set(None),
            crc32: Set(None),
            crc32c: Set(None),
            crc64nvme: Set(None),
            sha1: Set(None),
            sha256: Set(None),
            md5: Set(None),
            e_tag: Set(None),
            ..Default::default()
        };

        Version::insert(version).exec_with_returning(db).await
    }

    #[allow(dead_code)]
    #[deprecated]
    async fn update_delete_marker(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
        object_id: Uuid,
        versioning: bool,
    ) -> DbRes<Option<version::Model>> {
        PartMutation::delete_many(db, None, Some(id)).await?;

        let version = version::ActiveModel {
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(None),
            mime: Set(None),
            size: Set(None),
            crc32: Set(None),
            crc32c: Set(None),
            crc64nvme: Set(None),
            sha1: Set(None),
            sha256: Set(None),
            md5: Set(None),
            e_tag: Set(None),
            ..Default::default()
        };

        Version::update_many()
            .filter(version::Column::Id.eq(id))
            .col_expr(version::Column::UpdatedAt, Expr::current_timestamp().into())
            .set(version)
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }

    pub(super) async fn upsert_version_also_part(
        db: &impl ConnectionTrait,
        id: Option<Uuid>,
        object_id: Uuid,
        versioning: bool,
        mime: &Mime,
        read: impl Unpin + AsyncRead,
    ) -> InsRes<version::Model> {
        let id = id.unwrap_or_else(Uuid::new_v4);
        PartMutation::delete_many(db, None, Some(id)).await?;

        let part = PartMutation::upsert_with_chunk(db, None, Some(id), 1, Some(0), read).await?;

        let version = version::ActiveModel {
            id: Set(id),
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(0.into()),
            mime: Set(mime.to_string().into()),
            size: Set(part.size.into()),
            crc32: Set(part.crc32.into()),
            crc32c: Set(part.crc32c.into()),
            crc64nvme: Set(part.crc64nvme.into()),
            sha1: Set(part.sha1.into()),
            sha256: Set(part.sha256.into()),
            md5: Set(part.md5.into()),
            ..Default::default()
        };

        Ok(Version::insert(version)
            .on_conflict(
                OnConflict::column(version::Column::Id)
                    .target_and_where(version::Column::Versioning.eq(false))
                    .update_columns([
                        version::Column::Versioning,
                        version::Column::PartsCount,
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
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await?)
    }

    pub(super) async fn upsert_delete_marker_also_part(
        db: &impl ConnectionTrait,
        id: Option<Uuid>,
        object_id: Uuid,
        versioning: bool,
    ) -> DbRes<version::Model> {
        let id = id.unwrap_or_else(Uuid::new_v4);
        PartMutation::delete_many(db, None, Some(id)).await?;

        let version = version::ActiveModel {
            id: Set(id),
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(None),
            mime: Set(None),
            size: Set(None),
            crc32: Set(None),
            crc32c: Set(None),
            crc64nvme: Set(None),
            sha1: Set(None),
            sha256: Set(None),
            md5: Set(None),
            e_tag: Set(None),
            ..Default::default()
        };

        Version::insert(version)
            .on_conflict(
                OnConflict::column(version::Column::Id)
                    .target_and_where(version::Column::Versioning.eq(false))
                    .update_columns([
                        version::Column::Versioning,
                        version::Column::PartsCount,
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
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
    }

    pub(super) async fn delete(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
    ) -> DbRes<Option<version::Model>> {
        Version::delete_many()
            .filter(version::Column::Id.eq(id))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }
}
