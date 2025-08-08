use std::pin::pin;

use bytes::Bytes;
use bytesize::ByteSize;
use futures::Stream;
use futures::StreamExt;
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
use tokio_util::codec::FramedRead;
use tokio_util::io::StreamReader;
use uuid::Uuid;

use crate::InsRes;
use crate::VersionMutation;
use crate::VersionQuery;
use crate::error::DbRes;
use crate::utils::ChunkDecoder;
use crate::utils::get_mime;

pub struct ObjectQuery;

impl ObjectQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<object::Model>> {
        Object::find()
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .one(db)
            .await
    }

    pub async fn find_also_version(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
        version_id: Uuid,
    ) -> DbRes<Option<(object::Model, Option<version::Model>)>> {
        Object::find()
            .join(
                JoinType::LeftJoin,
                object::Relation::Version
                    .def()
                    .on_condition(move |_, _| version::Column::Id.eq(version_id).into_condition()),
            )
            .select_also(Version)
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .one(db)
            .await
    }

    pub async fn find_also_null_version(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<(object::Model, Option<version::Model>)>> {
        Object::find()
            .join(
                JoinType::LeftJoin,
                object::Relation::Version
                    .def()
                    .on_condition(|_, _| version::Column::Versioning.eq(false).into_condition()),
            )
            .select_also(Version)
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .order_by_desc(version::Column::CreatedAt)
            .one(db)
            .await
    }

    pub async fn find_both_latest_version(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<(object::Model, version::Model)>> {
        Object::find()
            .join(JoinType::InnerJoin, object::Relation::LatestVersion.def())
            .select_also(Version)
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .one_both(db)
            .await
    }

    pub async fn find_many_both_latest_version(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<(object::Model, version::Model)>>> {
        let mut query = Object::find()
            .join(JoinType::InnerJoin, object::Relation::LatestVersion.def())
            .select_also(Version)
            .filter(object::Column::BucketId.eq(bucket_id));
        if let Some(prefix) = prefix {
            query = query.filter(object::Column::Key.starts_with(prefix));
        }
        if let Some(continuation_token) = continuation_token {
            query = query.filter(object::Column::Key.gt(continuation_token));
        }
        query
            .filter(version::Column::PartsCount.is_not_null())
            .order_by_asc(object::Column::Key)
            .limit(limit)
            .stream_both(db)
            .await
    }
}

pub struct ObjectMutation;

impl ObjectMutation {
    async fn insert_also_delete_marker(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: String,
        versioning: bool,
    ) -> DbRes<(object::Model, version::Model)> {
        let version =
            VersionMutation::upsert_delete_marker_also_part(db, None, Uuid::new_v4(), versioning)
                .await?;

        let object = object::ActiveModel {
            id: Set(version.object_id),
            bucket_id: Set(bucket_id),
            key: Set(key),
            version_id: Set(version.id),
            ..Default::default()
        };

        let object = Object::insert(object).exec_with_returning(db).await?;

        Ok((object, version))
    }

    async fn update_version_id(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Uuid,
        version_id: Uuid,
    ) -> DbRes<Option<object::Model>> {
        let object = object::ActiveModel {
            version_id: Set(version_id),
            ..Default::default()
        };

        Object::update_many()
            .filter(object::Column::Id.eq(id))
            .set(object)
            .col_expr(object::Column::UpdatedAt, Expr::current_timestamp().into())
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }

    pub async fn update_also_delete_marker(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key: &str,
        versioning: bool,
    ) -> DbRes<Option<(object::Model, version::Model)>> {
        Ok(
            match ObjectQuery::find_both_latest_version(db, bucket_id, key).await? {
                Some((object, version)) => {
                    let version_id = (!versioning && !version.versioning).then_some(version.id);

                    let version = VersionMutation::upsert_delete_marker_also_part(
                        db, version_id, object.id, versioning,
                    )
                    .await?;
                    let object =
                        ObjectMutation::update_version_id(db, object.id, version.id).await?;

                    object.map(|object| (object, version))
                }
                None => None,
            },
        )
    }

    pub async fn upsert_also_version(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key: String,
        versioning: bool,
        mime: Option<Mime>,
        read: impl Unpin + AsyncRead,
    ) -> InsRes<(object::Model, version::Model)> {
        let (id, version_id) =
            match ObjectQuery::find_both_latest_version(db, bucket_id, &key).await? {
                Some((object, version)) => {
                    let version_id = (!versioning && !version.versioning).then_some(version.id);

                    (object.id, version_id)
                }
                None => (Uuid::new_v4(), None),
            };

        let decode = ChunkDecoder::with_capacity(ByteSize::kib(4).as_u64() as usize);
        let read = FramedRead::new(read, decode);
        let mut stream = pin!(read.peekable());

        let mime = if let Some(mime) = mime {
            mime
        } else {
            let chunk = match stream.as_mut().peek().await {
                Some(Ok(chunk)) => chunk,
                Some(Err(_)) => {
                    stream.try_next().await?;
                    unreachable!()
                }
                None => &Bytes::new(),
            };

            get_mime(&key, chunk)
        };

        let read = StreamReader::new(stream);
        let version =
            VersionMutation::upsert_version_also_part(db, version_id, id, versioning, &mime, read)
                .await?;

        let object = object::ActiveModel {
            id: Set(id),
            bucket_id: Set(bucket_id),
            key: Set(key),
            version_id: Set(version.id),
            ..Default::default()
        };

        let object = Object::insert(object)
            .on_conflict(
                OnConflict::columns([object::Column::BucketId, object::Column::Key])
                    .update_column(object::Column::VersionId)
                    .value(object::Column::UpdatedAt, Expr::current_timestamp())
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await?;

        Ok((object, version))
    }

    pub async fn upsert_also_delete_marker(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key: String,
        versioning: bool,
    ) -> DbRes<(object::Model, version::Model)> {
        match ObjectMutation::update_also_delete_marker(db, bucket_id, &key, versioning).await? {
            Some(object_version) => Ok(object_version),
            None => ObjectMutation::insert_also_delete_marker(db, bucket_id, key, versioning).await,
        }
    }

    pub async fn delete(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<object::Model>> {
        Object::delete_many()
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }

    pub async fn delete_also_version_nullable(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key: &str,
        version_id: Uuid,
    ) -> DbRes<Option<(object::Model, Option<version::Model>)>> {
        let mut object_version = if version_id.is_nil() {
            ObjectQuery::find_also_null_version(db, bucket_id, key).await?
        } else {
            ObjectQuery::find_also_version(db, bucket_id, key, version_id).await?
        };

        if let Some((object, Some(version))) = object_version {
            if object.version_id == version.id {
                if let Some(version) = VersionQuery::find_2nd_latest(db, object.id).await? {
                    if ObjectMutation::update_version_id(db, object.id, version.id)
                        .await?
                        .is_none()
                    {
                        todo!()
                    }
                }
            }

            let version = VersionMutation::delete(db, version_id).await?;
            object_version = Some((object, version));
        }

        Ok(object_version)
    }
}
