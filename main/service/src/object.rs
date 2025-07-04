use axum::body::BodyDataStream;
use crc_fast::CrcAlgorithm;
use digest::Digest;
use md5::Md5;
use minil_entity::object;
use minil_entity::prelude::*;
use sea_orm::*;
use sha1::Sha1;
use sha2::Sha256;
use tokio_stream::StreamExt;
use uuid::Uuid;

pub struct ObjectQuery;

impl ObjectQuery {
    pub async fn find_by_unique_id(
        db: &DbConn,
        bucket_id: Uuid,
        key: &str,
    ) -> Result<Option<object::Model>, DbErr> {
        Object::find()
            .filter(object::Column::BucketId.eq(bucket_id))
            .filter(object::Column::Key.eq(key))
            .one(db)
            .await
    }
}

pub struct ObjectMutation;

impl ObjectMutation {
    async fn insert(db: &DbConn, object: object::ActiveModel) -> Result<object::Model, DbErr> {
        TryInsert::one(object)
            .on_conflict(
                sea_query::OnConflict::columns([object::Column::BucketId, object::Column::Key])
                    .update_columns([
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
            .map(|res| match res {
                TryInsertResult::Inserted(bucket) => bucket,
                _ => unreachable!(),
            })
    }

    pub async fn create(
        db: &DbConn,
        bucket_id: Uuid,
        key: &str,
        mut stream: BodyDataStream,
    ) -> Result<Result<object::Model, DbErr>, axum::Error> {
        let mut size = 0u64;
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
            size += chunk.len() as u64;
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

    async fn delete(db: &DbConn, object: object::Model) -> Result<Option<object::Model>, DbErr> {
        Delete::one(object).exec_with_returning(db).await
    }

    pub async fn delete_by_unique_id(
        db: &DbConn,
        bucket_id: Uuid,
        key: &str,
    ) -> Result<Option<object::Model>, DbErr> {
        let object = ObjectQuery::find_by_unique_id(db, bucket_id, key).await?;
        match object {
            Some(object) => ObjectMutation::delete(db, object).await,
            None => Ok(None),
        }
    }
}
