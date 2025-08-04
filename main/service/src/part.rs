use std::pin::pin;

use bytesize::ByteSize;
use crc_fast::CrcAlgorithm;
use digest::DynDigest;
use digest::FixedOutput;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use md5::Md5;
use minil_entity::object;
use minil_entity::part;
use minil_entity::prelude::*;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_query::*;
use sha1::Sha1;
use sha2::Sha256;
use tokio::io::AsyncRead;
use tokio_util::codec::FramedRead;

use crate::ChunkMutation;
use crate::InsRes;
use crate::error::DbRes;
use crate::utils::ChunkDecoder;

pub struct PartQuery;

impl PartQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        upload_id: Option<Uuid>,
        version_id: Option<Uuid>,
        number: u16,
    ) -> DbRes<Option<part::Model>> {
        let mut query = Part::find();
        if let Some(upload_id) = upload_id {
            query = query.filter(part::Column::UploadId.eq(upload_id));
        }
        if let Some(version_id) = version_id {
            query = query.filter(part::Column::VersionId.eq(version_id));
        }
        query.filter(part::Column::Number.eq(number)).one(db).await
    }

    pub async fn find_by_version_id(
        db: &(impl ConnectionTrait + StreamTrait),
        version_id: Uuid,
        range: &Option<std::ops::RangeInclusive<u64>>,
    ) -> DbRes<impl Stream<Item = DbRes<part::Model>>> {
        let mut query = Part::find().filter(part::Column::VersionId.eq(version_id));
        if let Some(range) = range {
            query = query
                .filter(part::Column::Start.lte(*range.end()))
                .filter(part::Column::End.gte(*range.start()));
        }
        query.order_by_asc(part::Column::Number).stream(db).await
    }
}

pub struct PartMutation;

impl PartMutation {
    pub async fn upsert_with_chunk(
        db: &impl ConnectionTrait,
        upload_id: Option<Uuid>,
        version_id: Option<Uuid>,
        number: u16,
        start: Option<u64>,
        read: impl Unpin + AsyncRead,
    ) -> InsRes<part::Model> {
        let id = PartQuery::find(db, upload_id, version_id, number)
            .await?
            .map_or_else(Uuid::new_v4, |part| part.id);
        ChunkMutation::delete_by_part_id(db, id).await?;

        let decode = ChunkDecoder::with_capacity(ByteSize::mib(4).as_u64() as usize);
        let read = FramedRead::new(read, decode)
            .enumerate()
            .map(|(index, chunk)| chunk.map(|chunk| (index as u64, chunk)));
        let mut stream = pin!(read);

        let mut size = 0u64;
        let mut crc32;
        let mut crc32c;
        let mut crc64nvme;
        let mut sha1;
        let mut sha256;
        let mut md5;
        {
            use crc_fast::Digest;
            crc32 = Digest::new(CrcAlgorithm::Crc32IsoHdlc);
            crc32c = Digest::new(CrcAlgorithm::Crc32Iscsi);
            crc64nvme = Digest::new(CrcAlgorithm::Crc64Nvme);
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
            crc32c.update(&chunk);
            crc64nvme.update(&chunk);
            sha1.update(&chunk);
            sha256.update(&chunk);
            md5.update(&chunk);

            let end = size - 1;
            ChunkMutation::insert(db, id, index, start, end, chunk.to_vec()).await?;
        }

        let part = part::ActiveModel {
            id: Set(id),
            upload_id: Set(upload_id),
            version_id: Set(version_id),
            number: Set(number as i16),
            start: Set(start.map(|start| start as i64)),
            end: Set(start.map(|start| (start + size - 1) as i64)),
            size: Set(size as i64),
            crc32: Set(Box::new(crc32).finalize().to_vec()),
            crc32c: Set(Box::new(crc32c).finalize().to_vec()),
            crc64nvme: Set(Box::new(crc64nvme).finalize().to_vec()),
            sha1: Set(sha1.finalize_fixed().to_vec()),
            sha256: Set(sha256.finalize_fixed().to_vec()),
            md5: Set(md5.finalize_fixed().to_vec()),
            ..Default::default()
        };

        Ok(Part::insert(part)
            .on_conflict(
                OnConflict::column(part::Column::Id)
                    .update_columns([
                        part::Column::Size,
                        part::Column::Crc32,
                        part::Column::Crc32c,
                        part::Column::Crc64nvme,
                        part::Column::Sha1,
                        part::Column::Sha256,
                        part::Column::Md5,
                    ])
                    .value(object::Column::UpdatedAt, Expr::current_timestamp())
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await?)
    }

    pub(super) async fn delete_by_version_id(
        db: &impl ConnectionTrait,
        version_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Part::delete_many()
            .filter(part::Column::VersionId.eq(version_id))
            .exec(db)
            .await
    }
}
