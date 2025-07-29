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
use sea_query::*;
use tokio::io::AsyncRead;
use tokio_util::codec::FramedRead;
use tokio_util::io::StreamReader;
use uuid::Uuid;

use crate::InsRes;
use crate::VersionMutation;
use crate::error::DbRes;
use crate::utils::ChunkDecoder;
use crate::utils::DeleteManyExt;
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

    pub async fn find_version(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<(object::Model, version::Model)>> {
        Ok(Object::find()
            .join(JoinType::InnerJoin, object::Relation::LatestVersion.def())
            .select_also(Version)
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .filter(version::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .map(|(object, version)| (object, version.unwrap_or_else(|| unreachable!()))))
    }

    pub async fn find_version_by_bucket_id(
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
        Ok(query
            .filter(version::Column::DeletedAt.is_null())
            .order_by_asc(object::Column::Key)
            .limit(limit)
            .stream(db)
            .await?
            .map(|res| {
                res.map(|(object, version)| (object, version.unwrap_or_else(|| unreachable!())))
            }))
    }
}

pub struct ObjectMutation;

impl ObjectMutation {
    pub async fn insert_version(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key: String,
        mime: Option<Mime>,
        read: impl Unpin + AsyncRead,
    ) -> InsRes<(object::Model, version::Model)> {
        let decode = ChunkDecoder::with_capacity(ByteSize::kib(4).as_u64() as usize);
        let read = FramedRead::new(read, decode);
        let mut stream = pin!(read.peekable());

        let mime = match mime {
            Some(mime) => mime,
            None => {
                let chunk = match stream.as_mut().peek().await {
                    Some(Ok(chunk)) => chunk,
                    Some(Err(_)) => {
                        stream.try_next().await?;
                        unreachable!()
                    }
                    None => &Bytes::new(),
                };

                get_mime(&key, chunk)
            }
        };

        let object = object::ActiveModel {
            id: Set(Uuid::new_v4()),
            bucket_id: Set(bucket_id),
            key: Set(key.clone()),
            version_id: Set(Uuid::new_v4()),
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

        let read = StreamReader::new(stream);
        let version = VersionMutation::insert(db, object.version_id, object.id, mime, read).await?;

        Ok((object, version))
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
}
