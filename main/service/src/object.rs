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
    async fn insert(
        db: &DbConn,
        object: object::ActiveModel,
    ) -> Result<Option<object::Model>, DbErr> {
        TryInsert::one(object)
            .on_conflict(
                sea_query::OnConflict::columns([object::Column::BucketId, object::Column::Key])
                    .do_nothing()
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
            .or_else(|err| match err {
                DbErr::RecordNotFound(_) => Ok(TryInsertResult::Conflicted),
                _ => Err(err),
            })
            .map(|res| match res {
                TryInsertResult::Empty => unreachable!(),
                TryInsertResult::Conflicted => None,
                TryInsertResult::Inserted(bucket) => Some(bucket),
            })
    }

    pub async fn create(
        db: &DbConn,
        bucket_id: Uuid,
        key: &str,
        mut stream: BodyDataStream,
    ) -> Result<Result<Option<object::Model>, DbErr>, axum::Error> {
        let mut size = 0u64;
        let mut crc32 = crc_fast::Digest::new(CrcAlgorithm::Crc32IsoHdlc);
        let mut crc32c = crc_fast::Digest::new(CrcAlgorithm::Crc32Iscsi);
        let mut crc64nvme = crc_fast::Digest::new(CrcAlgorithm::Crc64Nvme);
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

        let object = object::ActiveModel {
            id: Set(Uuid::new_v4()),
            bucket_id: Set(bucket_id),
            key: Set(key.to_owned()),
            size: Set(size),
            crc32: Set(crc32.finalize() as u32),
            crc32c: Set(crc32c.finalize() as u32),
            crc64nvme: Set(crc64nvme.finalize()),
            sha1: Set(sha1.finalize().to_vec()),
            sha256: Set(sha256.finalize().to_vec()),
            md5: Set(md5.finalize().to_vec()),
            ..Default::default()
        };
        Ok(ObjectMutation::insert(db, object.to_owned()).await)
    }
}
