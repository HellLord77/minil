use std::ops::RangeInclusive;
use std::pin::pin;

use bytesize::ByteSize;
use crc_fast::CrcAlgorithm;
use digest::DynDigest;
use digest::FixedOutput;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use md5::Md5;
use minil_entity::prelude::*;
use minil_entity::upload_part;
use minil_entity::version_part;
use sea_orm::prelude::*;
use sea_orm::*;
use sha1::Sha1;
use sha2::Sha256;
use tokio::io::AsyncRead;
use tokio_util::codec::FramedRead;

use crate::ChunkMutation;
use crate::InsRes;
use crate::error::DbRes;
use crate::utils::ChunkDecoder;

pub struct VersionPartQuery;

impl VersionPartQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        version_id: Uuid,
        number: u16,
    ) -> DbRes<Option<version_part::Model>> {
        VersionPart::find()
            .filter(version_part::Column::VersionId.eq(version_id))
            .filter(version_part::Column::Number.eq(number))
            .one(db)
            .await
    }

    pub(super) async fn find_many_ranged(
        db: &(impl ConnectionTrait + StreamTrait),
        version_id: Uuid,
        range: Option<&RangeInclusive<u64>>,
    ) -> DbRes<impl Stream<Item = DbRes<version_part::Model>>> {
        VersionPart::find()
            .filter(version_part::Column::VersionId.eq(version_id))
            .apply_if(range, |query, range| {
                query
                    .filter(version_part::Column::Start.lte(*range.end()))
                    .filter(version_part::Column::End.gte(*range.start()))
            })
            .order_by_asc(version_part::Column::Number)
            .stream(db)
            .await
    }
}

pub struct VersionPartMutation;

impl VersionPartMutation {
    pub(super) async fn insert_with_chunk(
        db: &impl ConnectionTrait,
        version_id: Uuid,
        read: impl AsyncRead,
    ) -> InsRes<version_part::Model> {
        let id = Uuid::new_v4();

        let decode = ChunkDecoder::with_capacity(ByteSize::mib(4).as_u64() as usize);
        let read = FramedRead::new(read, decode)
            .enumerate()
            .map(|(index, chunk)| chunk.map(|chunk| (index as u64, chunk)));
        let mut stream = pin!(read);

        let mut size = 0u64;
        let mut crc32;
        let mut crc32_c;
        let mut crc64_nvme;
        let mut sha1;
        let mut sha256;
        let mut md5;
        {
            use crc_fast::Digest;
            crc32 = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
            crc32_c = Digest::new(CrcAlgorithm::Crc32Iscsi);
            crc64_nvme = Digest::new(CrcAlgorithm::Crc64Nvme);
        }
        {
            use digest::Digest;
            sha1 = Sha1::new();
            sha256 = Sha256::new();
            md5 = Md5::new();
        }

        while let Some((index, chunk)) = stream.try_next().await? {
            let start = size;

            size += chunk.len() as u64;
            crc32.update(&chunk);
            crc32_c.update(&chunk);
            crc64_nvme.update(&chunk);
            sha1.update(&chunk);
            sha256.update(&chunk);
            md5.update(&chunk);

            let end = size.saturating_sub(1);
            ChunkMutation::insert(db, None, Some(id), index, start, end, chunk.to_vec()).await?;
        }

        let part = version_part::ActiveModel {
            id: Set(id),
            version_id: Set(version_id),
            number: Set(1),
            start: Set(0),
            end: Set(size.saturating_sub(1) as i64),
            size: Set(size as i64),
            crc32: Set(Box::new(crc32).finalize().to_vec()), // fixme
            crc32_c: Set(Box::new(crc32_c).finalize().to_vec()), // fixme
            crc64_nvme: Set(Box::new(crc64_nvme).finalize().to_vec()), // fixme
            sha1: Set(sha1.finalize_fixed().to_vec()),
            sha256: Set(sha256.finalize_fixed().to_vec()),
            md5: Set(md5.finalize_fixed().to_vec()),
            ..Default::default()
        };

        Ok(VersionPart::insert(part).exec_with_returning(db).await?)
    }

    pub(super) async fn insert_many_from_upload_parts(
        db: &impl ConnectionTrait,
        version_id: Uuid,
        iter: impl Iterator<Item = upload_part::Model>,
    ) -> DbRes<InsertResult<version_part::ActiveModel>> {
        let mut size = 0u64;
        let mut parts = vec![];

        for (index, upload_part) in iter.enumerate() {
            let id = Uuid::new_v4();

            ChunkMutation::update_many_version_part_id_by_upload_part_id(db, upload_part.id, id)
                .await?;

            let start = size;
            size += upload_part.size as u64;
            let end = size.saturating_sub(1);

            let part = version_part::ActiveModel {
                id: Set(id),
                version_id: Set(version_id),
                number: Set((index + 1) as i16),
                start: Set(start as i64),
                end: Set(end as i64),
                size: Set(upload_part.size),
                crc32: Set(upload_part.crc32),
                crc32_c: Set(upload_part.crc32_c),
                crc64_nvme: Set(upload_part.crc64_nvme),
                sha1: Set(upload_part.sha1),
                sha256: Set(upload_part.sha256),
                md5: Set(upload_part.md5),
                ..Default::default()
            };

            parts.push(part);
        }

        VersionPart::insert_many(parts).exec(db).await
    }

    pub(super) async fn delete_many(
        db: &impl ConnectionTrait,
        version_id: Uuid,
    ) -> DbRes<DeleteResult> {
        VersionPart::delete_many()
            .filter(version_part::Column::VersionId.eq(version_id))
            .exec(db)
            .await
    }
}
