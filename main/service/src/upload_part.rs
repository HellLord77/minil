use std::pin::pin;

use async_stream::try_stream;
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
use sea_orm::prelude::*;
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use sha1::Sha1;
use sha2::Sha256;
use tokio::io::AsyncRead;
use tokio_util::codec::FramedRead;

use crate::ChunkMutation;
use crate::InsRes;
use crate::error::DbRes;
use crate::utils::ChunkDecoder;
use crate::utils::DigestExt;

pub struct UploadPartQuery;

impl UploadPartQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        upload_id: Uuid,
        number: u16,
    ) -> DbRes<Option<upload_part::Model>> {
        UploadPart::find()
            .filter(upload_part::Column::UploadId.eq(upload_id))
            .filter(upload_part::Column::Number.eq(number))
            .one(db)
            .await
    }

    async fn find_filtered(
        db: &impl ConnectionTrait,
        upload_id: Uuid,
        number: u16,
        crc32: Option<Vec<u8>>,
        crc32_c: Option<Vec<u8>>,
        crc64_nvme: Option<Vec<u8>>,
        sha1: Option<Vec<u8>>,
        sha256: Option<Vec<u8>>,
        md5: Option<Vec<u8>>,
    ) -> DbRes<Option<upload_part::Model>> {
        UploadPart::find()
            .filter(upload_part::Column::UploadId.eq(upload_id))
            .filter(upload_part::Column::Number.eq(number))
            .apply_if(crc32, |query, crc32| {
                query.filter(upload_part::Column::Crc32.eq(crc32))
            })
            .apply_if(crc32_c, |query, crc32_c| {
                query.filter(upload_part::Column::Crc32C.eq(crc32_c))
            })
            .apply_if(crc64_nvme, |query, crc64_nvme| {
                query.filter(upload_part::Column::Crc64Nvme.eq(crc64_nvme))
            })
            .apply_if(sha1, |query, sha1| {
                query.filter(upload_part::Column::Sha1.eq(sha1))
            })
            .apply_if(sha256, |query, sha256| {
                query.filter(upload_part::Column::Sha256.eq(sha256))
            })
            .apply_if(md5, |query, md5| {
                query.filter(upload_part::Column::Md5.eq(md5))
            })
            .one(db)
            .await
    }

    pub async fn find_many(
        db: &(impl ConnectionTrait + StreamTrait),
        upload_id: Uuid,
        part_number_marker: Option<u16>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<upload_part::Model>>> {
        UploadPart::find()
            .filter(upload_part::Column::UploadId.eq(upload_id))
            .apply_if(part_number_marker, |query, part_number_marker| {
                query.filter(upload_part::Column::Number.gt(part_number_marker))
            })
            .order_by_asc(upload_part::Column::Number)
            .limit(limit)
            .stream(db)
            .await
    }

    pub fn find_many_filtered(
        db: &impl ConnectionTrait,
        upload_id: Uuid,
        iter: impl Iterator<
            Item = (
                u16,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
            ),
        >,
    ) -> impl Stream<Item = DbRes<Option<upload_part::Model>>> {
        // fixme union
        try_stream! {
            for (number, crc32, crc32_c, crc64_nvme, sha1, sha256, md5) in iter {
                yield UploadPartQuery::find_filtered(
                    db, upload_id, number, crc32, crc32_c, crc64_nvme, sha1, sha256, md5
                ).await?
            }
        }
    }
}

pub struct UploadPartMutation;

impl UploadPartMutation {
    pub async fn upsert_with_chunk(
        db: &impl ConnectionTrait,
        upload_id: Uuid,
        number: u16,
        read: impl AsyncRead,
    ) -> InsRes<upload_part::Model> {
        let id = if let Some(part) = UploadPartQuery::find(db, upload_id, number).await? {
            ChunkMutation::delete_many_by_upload_part_id(db, part.id).await?;

            part.id
        } else {
            Uuid::new_v4()
        };

        let decode = ChunkDecoder::with_capacity(ByteSize::mib(5).as_u64() as usize);
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
            ChunkMutation::insert(db, Some(id), None, index, start, end, chunk.to_vec()).await?;
        }

        let part = upload_part::ActiveModel {
            id: Set(id),
            upload_id: Set(upload_id),
            number: Set(number as i16),
            size: Set(size as i64),
            crc32: Set(crc32.finalize_vec()),
            crc32_c: Set(crc32_c.finalize_vec()),
            crc64_nvme: Set(crc64_nvme.finalize_vec()),
            sha1: Set(sha1.finalize_fixed().to_vec()),
            sha256: Set(sha256.finalize_fixed().to_vec()),
            md5: Set(md5.finalize_fixed().to_vec()),
            ..Default::default()
        };

        Ok(UploadPart::insert(part)
            .on_conflict(
                OnConflict::column(upload_part::Column::Id)
                    .update_columns([
                        upload_part::Column::Size,
                        upload_part::Column::Crc32,
                        upload_part::Column::Crc32C,
                        upload_part::Column::Crc64Nvme,
                        upload_part::Column::Sha1,
                        upload_part::Column::Sha256,
                        upload_part::Column::Md5,
                    ])
                    .value(upload_part::Column::UpdatedAt, Expr::current_timestamp())
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await?)
    }
}
