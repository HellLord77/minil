use crc_fast::CrcAlgorithm;
use crc_fast::checksum_combine;
use digest::DynDigest;
use digest::FixedOutput;
use futures::Stream;
use futures::TryStreamExt;
use md5::Md5;
use mime::Mime;
use minil_entity::object;
use minil_entity::prelude::*;
use minil_entity::upload_part;
use minil_entity::version;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_ext::prelude::*;
use sea_query::*;
use tokio::io::AsyncRead;

use crate::InsRes;
use crate::VersionPartMutation;
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
        Version::find()
            .find_both_related(Object)
            .filter(object::Column::BucketId.eq(bucket_id))
            .apply_if(prefix, |query, prefix| {
                query.filter(object::Column::Key.starts_with(prefix))
            })
            .apply_if(key_maker, |query, key_marker| {
                if version_id_marker.is_some() {
                    query.filter(object::Column::Key.gte(key_marker))
                } else {
                    query.filter(object::Column::Key.gt(key_marker))
                }
            })
            .apply_if(version_id_marker, |query, version_id_marker| {
                query.filter(version::Column::Id.gt(version_id_marker))
            })
            .order_by_asc(object::Column::Key)
            .order_by_desc(version::Column::CreatedAt)
            .limit(limit)
            .stream_both(db)
            .await
    }
}

pub struct VersionMutation;

impl VersionMutation {
    pub(super) async fn upsert_version_also_part(
        db: &impl ConnectionTrait,
        id: Option<Uuid>,
        object_id: Uuid,
        versioning: bool,
        mime: Option<&Mime>,
        read: impl AsyncRead,
    ) -> InsRes<version::Model> {
        let id = if let Some(id) = id {
            VersionPartMutation::delete_many(db, id).await?;

            id
        } else {
            Uuid::new_v4()
        };

        let part = VersionPartMutation::insert_with_chunk(db, id, read).await?;

        let version = version::ActiveModel {
            id: Set(id),
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(Some(0)),
            mime: Set(mime.map(ToString::to_string)),
            size: Set(Some(part.size)),
            crc32: Set(Some(part.crc32)),
            crc32_c: Set(Some(part.crc32_c)),
            crc64_nvme: Set(Some(part.crc64_nvme)),
            sha1: Set(Some(part.sha1)),
            sha256: Set(Some(part.sha256)),
            md5: Set(Some(part.md5)),
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
                        version::Column::Crc32C,
                        version::Column::Crc64Nvme,
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
        let id = if let Some(id) = id {
            VersionPartMutation::delete_many(db, id).await?;

            id
        } else {
            Uuid::new_v4()
        };

        let version = version::ActiveModel {
            id: Set(id),
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(None),
            mime: Set(None),
            size: Set(None),
            crc32: Set(None),
            crc32_c: Set(None),
            crc64_nvme: Set(None),
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
                        version::Column::Crc32C,
                        version::Column::Crc64Nvme,
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

    pub(super) async fn upsert_version_with_part_from_upload_parts(
        db: &(impl ConnectionTrait + StreamTrait),
        id: Option<Uuid>,
        object_id: Uuid,
        versioning: bool,
        mime: Option<&Mime>,
        iter: impl Iterator<Item = upload_part::Model>,
    ) -> DbRes<version::Model> {
        let id = if let Some(id) = id {
            VersionPartMutation::delete_many(db, id).await?;

            id
        } else {
            Uuid::new_v4()
        };

        let mut parts_count = 0u16;
        let mut size = 0;
        let mut crc32_digest = 0;
        let mut crc32_c_digest = 0;
        let mut crc64_nvme_digest = 0;
        let mut sha1_digest = None;
        let mut sha256_digest = None;
        let mut md5_digest = None;
        let mut e_tag;
        {
            use digest::Digest;
            e_tag = Md5::new();
        }

        let mut iter = iter.peekable();
        if let Some(part) = iter.peek() {
            sha1_digest = Some(part.sha1.clone());
            sha256_digest = Some(part.sha256.clone());
            md5_digest = Some(part.md5.clone());
        }

        let iter = iter.inspect(|part| {
            let part_size = part.size as u64;
            parts_count += 1;
            size += part_size;

            let mut crc32 = vec![0; 4];
            let mut crc32_c = vec![0; 4];
            crc32.extend_from_slice(&part.crc32);
            crc32_c.extend_from_slice(&part.crc32_c);
            crc32_digest = checksum_combine(
                CrcAlgorithm::Crc32IsoHdlc,
                crc32_digest,
                u64::from_be_bytes(crc32.try_into().unwrap()),
                part_size,
            );
            crc32_c_digest = checksum_combine(
                CrcAlgorithm::Crc32Iscsi,
                crc32_c_digest,
                u64::from_be_bytes(crc32_c.try_into().unwrap()),
                part_size,
            );
            crc64_nvme_digest = checksum_combine(
                CrcAlgorithm::Crc64Nvme,
                crc64_nvme_digest,
                u64::from_be_bytes(part.crc64_nvme.clone().try_into().unwrap()),
                part_size,
            );

            e_tag.update(&part.md5);
        });

        VersionPartMutation::insert_many_from_upload_parts(db, id, iter).await?;

        let version = version::ActiveModel {
            id: Set(id),
            object_id: Set(object_id),
            versioning: Set(versioning),
            parts_count: Set(Some(parts_count as i16)),
            mime: Set(mime.map(ToString::to_string)),
            size: Set(Some(size as i64)),
            crc32: Set(Some(crc32_digest.to_be_bytes()[4..].to_vec())),
            crc32_c: Set(Some(crc32_c_digest.to_be_bytes()[4..].to_vec())),
            crc64_nvme: Set(Some(crc64_nvme_digest.to_be_bytes().to_vec())),
            sha1: Set(sha1_digest.filter(|_| parts_count == 1)), // fixme
            sha256: Set(sha256_digest.filter(|_| parts_count == 1)), // fixme
            md5: Set(md5_digest.filter(|_| parts_count == 1)),   // fixme
            e_tag: Set((parts_count != 1)
                .then(|| format!("\"{}-{parts_count}\"", hex::encode(e_tag.finalize_fixed())))),
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
                        version::Column::Crc32C,
                        version::Column::Crc64Nvme,
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
        object_id: Uuid,
    ) -> DbRes<Option<version::Model>> {
        Version::delete_many()
            .filter(version::Column::Id.eq(id))
            .filter(version::Column::ObjectId.eq(object_id))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }
}
