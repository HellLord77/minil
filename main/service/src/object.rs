use std::io;

use bytes::Bytes;
use crc_fast::CrcAlgorithm;
use digest::Digest;
use md5::Md5;
use minil_entity::object;
use minil_entity::prelude::*;
use sea_orm::*;
use sha1::Sha1;
use sha2::Sha256;
use tokio::io::AsyncRead;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;
use uuid::Uuid;

use crate::error::DbRes;
use crate::utils::ChunkedDecoder;
use crate::utils::get_mime;

pub struct ObjectQuery;

impl ObjectQuery {
    pub async fn find_by_unique_id(
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

    pub async fn find_all_by_bucket_id(
        db: &(impl ConnectionTrait + StreamTrait),
        bucket_id: Uuid,
        key_starts_with: Option<&str>,
        key_gt: Option<&str>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<object::Model>>> {
        let mut query = Object::find().filter(object::Column::BucketId.eq(bucket_id));
        if let Some(key_starts_with) = key_starts_with {
            query = query.filter(object::Column::Key.starts_with(key_starts_with));
        }
        if let Some(key_gt) = key_gt {
            query = query.filter(object::Column::Key.gt(key_gt));
        }
        query
            .order_by_asc(object::Column::Key)
            .limit(limit)
            .stream(db)
            .await
    }
}

pub struct ObjectMutation;

impl ObjectMutation {
    async fn insert(
        db: &impl ConnectionTrait,
        object: object::ActiveModel,
    ) -> DbRes<object::Model> {
        Insert::one(object)
            .on_conflict(
                sea_query::OnConflict::columns([object::Column::BucketId, object::Column::Key])
                    .update_columns([
                        object::Column::Mime,
                        object::Column::Size,
                        object::Column::Crc32,
                        object::Column::Crc32c,
                        object::Column::Crc64nvme,
                        object::Column::Sha1,
                        object::Column::Sha256,
                        object::Column::Md5,
                    ])
                    .value(
                        object::Column::UpdatedAt,
                        sea_query::Expr::current_timestamp(),
                    )
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
    }

    pub async fn create(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
        mime: Option<&str>,
        read: impl Unpin + AsyncRead,
    ) -> io::Result<DbRes<object::Model>> {
        let mut stream = FramedRead::new(
            read,
            ChunkedDecoder::with_capacity(bytesize::mib(4u64) as usize),
        )
        .peekable();

        let mime = match mime {
            Some(mime) => mime.to_owned(),
            None => {
                let chunk = match stream.peek().await {
                    Some(Ok(chunk)) => chunk,
                    Some(Err(_)) => {
                        stream.try_next().await?;
                        unreachable!()
                    }
                    None => &Bytes::new(),
                };

                get_mime(key, chunk).essence_str().to_owned()
            }
        };

        let mut size = 0usize;
        let mut crc32;
        let mut crc32c;
        let mut crc64nvme;
        {
            use crc_fast::Digest;
            crc32 = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
            crc32c = Digest::new(CrcAlgorithm::Crc32Iscsi);
            crc64nvme = Digest::new(CrcAlgorithm::Crc64Nvme);
        }
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        let mut md5 = Md5::new();

        while let Some(chunk) = stream.try_next().await? {
            size += chunk.len();
            crc32.update(&chunk);
            crc32c.update(&chunk);
            crc64nvme.update(&chunk);
            sha1.update(&chunk);
            sha256.update(&chunk);
            md5.update(&chunk);
        }

        let mut crc32_buf = [0u8; 4];
        let mut crc32c_buf = [0u8; 4];
        let mut crc64nvme_buf = [0u8; 8];
        {
            use digest::DynDigest;
            crc32
                .finalize_into(&mut crc32_buf)
                .unwrap_or_else(|_err| unreachable!());
            crc32c
                .finalize_into(&mut crc32c_buf)
                .unwrap_or_else(|_err| unreachable!());
            crc64nvme
                .finalize_into(&mut crc64nvme_buf)
                .unwrap_or_else(|_err| unreachable!());
        }

        let object = object::ActiveModel {
            id: Set(Uuid::new_v4()),
            bucket_id: Set(bucket_id),
            key: Set(key.to_owned()),
            mime: Set(mime),
            size: Set(size as i64),
            crc32: Set(crc32_buf.to_vec()),
            crc32c: Set(crc32c_buf.to_vec()),
            crc64nvme: Set(crc64nvme_buf.to_vec()),
            sha1: Set(sha1.finalize().to_vec()),
            sha256: Set(sha256.finalize().to_vec()),
            md5: Set(md5.finalize().to_vec()),
            ..Default::default()
        };
        Ok(ObjectMutation::insert(db, object).await)
    }

    async fn delete(
        db: &impl ConnectionTrait,
        object: object::Model,
    ) -> DbRes<Option<object::Model>> {
        Delete::one(object).exec_with_returning(db).await
    }

    pub async fn delete_by_unique_id(
        db: &impl ConnectionTrait,
        bucket_id: Uuid,
        key: &str,
    ) -> DbRes<Option<object::Model>> {
        let object = ObjectQuery::find_by_unique_id(db, bucket_id, key).await?;
        match object {
            Some(object) => ObjectMutation::delete(db, object).await,
            None => Ok(None),
        }
    }
}
