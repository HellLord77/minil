use std::ops::RangeInclusive;

use async_stream::try_stream;
use bytes::Bytes;
use futures::Stream;
use minil_entity::chunk;
use minil_entity::object;
use minil_entity::prelude::*;
use sea_orm::prelude::Expr;
use sea_orm::*;
use uuid::Uuid;

use crate::VersionPartQuery;
use crate::error::DbRes;

pub struct ChunkQuery;

impl ChunkQuery {
    async fn find_many_by_version_part_id(
        db: &(impl ConnectionTrait + StreamTrait),
        version_part_id: Uuid,
        range: Option<&RangeInclusive<u64>>,
    ) -> DbRes<impl Stream<Item = DbRes<chunk::Model>>> {
        Chunk::find()
            .filter(chunk::Column::VersionPartId.eq(version_part_id))
            .apply_if(range, |query, range| {
                query
                    .filter(chunk::Column::Start.lte(*range.end()))
                    .filter(chunk::Column::End.gte(*range.start()))
            })
            .order_by_asc(chunk::Column::Index)
            .stream(db)
            .await
    }

    //noinspection RsBorrowChecker
    pub fn find_many_ranged_part_data_by_version_part_id(
        db: impl ConnectionTrait + StreamTrait,
        version_part_id: Uuid,
        range: Option<RangeInclusive<u64>>,
    ) -> impl Stream<Item = DbRes<Bytes>> {
        try_stream! {
            let chunks = ChunkQuery::find_many_by_version_part_id(&db, version_part_id, range.as_ref()).await?;
            for await chunk in chunks {
                let chunk = chunk?;

                let range = range.as_ref().map(|range| {
                    let start = chunk.start as u64;
                    let end = chunk.end as u64;
                    let offset = start;

                    let start = (start.max(*range.start()) - offset) as usize;
                    let end = (end.min(*range.end()) - offset) as usize;
                    start..=end
                });

                yield Bytes::copy_from_slice(range.map_or(&chunk.data, |range| &chunk.data[range]))
            }
        }
    }

    //noinspection RsBorrowChecker
    pub fn find_many_ranged_version_data_by_version_id(
        db: impl Clone + ConnectionTrait + StreamTrait,
        version_id: Uuid,
        range: Option<RangeInclusive<u64>>,
    ) -> impl Stream<Item = DbRes<Bytes>> {
        try_stream! {
            let parts = VersionPartQuery::find_many_ranged(&db, version_id, range.as_ref()).await?;
            for await part in parts {
                let part = part?;

                let range = range.as_ref().map(|range| {
                    let start = part.start as u64;
                    let end = part.end as u64;
                    let offset = start;

                    let start = start.max(*range.start()) - offset;
                    let end = end.min(*range.end()) - offset;
                    start..=end
                });

                let chunks = ChunkQuery::find_many_ranged_part_data_by_version_part_id(db.clone(), part.id, range);
                for await chunk in chunks {
                    yield chunk?
                }
            }
        }
    }
}

pub struct ChunkMutation;

impl ChunkMutation {
    pub(super) async fn insert(
        db: &impl ConnectionTrait,
        upload_part_id: Option<Uuid>,
        version_part_id: Option<Uuid>,
        index: u64,
        start: u64,
        end: u64,
        data: Vec<u8>,
    ) -> DbRes<InsertResult<chunk::ActiveModel>> {
        let chunk = chunk::ActiveModel {
            id: Set(Uuid::new_v4()),
            upload_part_id: Set(upload_part_id),
            version_part_id: Set(version_part_id),
            index: Set(index as i64),
            start: Set(start as i64),
            end: Set(end as i64),
            data: Set(data),
            ..Default::default()
        };

        Chunk::insert(chunk).exec(db).await
    }

    pub(super) async fn update_many_version_part_id_by_upload_part_id(
        db: &impl ConnectionTrait,
        upload_part_id: Uuid,
        version_part_id: Uuid,
    ) -> DbRes<UpdateResult> {
        let chunk = chunk::ActiveModel {
            upload_part_id: Set(None),
            version_part_id: Set(Some(version_part_id)),
            ..Default::default()
        };

        Chunk::update_many()
            .filter(chunk::Column::UploadPartId.eq(upload_part_id))
            .set(chunk)
            .col_expr(object::Column::UpdatedAt, Expr::current_timestamp().into())
            .exec(db)
            .await
    }

    pub(super) async fn delete_many_by_upload_part_id(
        db: &impl ConnectionTrait,
        upload_part_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Chunk::delete_many()
            .filter(chunk::Column::UploadPartId.eq(upload_part_id))
            .exec(db)
            .await
    }
}
