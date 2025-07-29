use std::ops::RangeInclusive;
use std::pin::pin;

use async_stream::try_stream;
use bytes::Bytes;
use futures::Stream;
use futures::TryStreamExt;
use minil_entity::chunk;
use minil_entity::part;
use minil_entity::prelude::*;
use sea_orm::*;
use uuid::Uuid;

use crate::error::DbRes;

pub struct ChunkQuery;

impl ChunkQuery {
    pub async fn find(
        db: &(impl ConnectionTrait + StreamTrait),
        part_id: Uuid,
        index: u64,
    ) -> DbRes<Option<chunk::Model>> {
        Chunk::find()
            .filter(chunk::Column::PartId.eq(part_id))
            .filter(chunk::Column::Id.eq(index))
            .one(db)
            .await
    }

    pub async fn stream_by_version_id(
        db: impl Clone + ConnectionTrait + StreamTrait,
        version_id: Uuid,
        range: Option<RangeInclusive<u64>>,
    ) -> impl Stream<Item = DbRes<Bytes>> {
        try_stream! {
            let mut query = Part::find().filter(part::Column::VersionId.eq(version_id));
            if let Some(range) = &range {
                query = query
                    .filter(part::Column::Start.lte(*range.end()))
                    .filter(part::Column::End.gte(*range.start()));
            }
            let parts = query
                .order_by_asc(part::Column::Number)
                .stream(&db)
                .await?;
            let mut stream = pin!(parts);

            while let Some(part) = stream.try_next().await? {
                let range = range.as_ref().map(|range| {
                    let start = part.start.unwrap_or_else(|| unreachable!()) as u64;
                    let end = part.end.unwrap_or_else(|| unreachable!()) as u64;
                    let offset = start;

                    let start = start.max(*range.start()) - offset;
                    let end = end.min(*range.end()) - offset;
                    start..=end
                });

                let chunks = ChunkQuery::stream_by_part_id(db.clone(), part.id, range).await;
                for await chunk in chunks {
                    yield chunk?;
                }
            }
        }
    }

    pub async fn stream_by_part_id(
        db: impl ConnectionTrait + StreamTrait,
        part_id: Uuid,
        range: Option<RangeInclusive<u64>>,
    ) -> impl Stream<Item = DbRes<Bytes>> {
        try_stream! {
            let mut query = Chunk::find()
                .filter(chunk::Column::PartId.eq(part_id));
            if let Some(range) = &range {
                query = query
                    .filter(chunk::Column::Start.lte(*range.end()))
                    .filter(chunk::Column::End.gte(*range.start()));
            }
            let chunks = query
                .order_by_asc(chunk::Column::Index)
                .stream(&db)
                .await?;
            let mut stream = pin!(chunks);

            while let Some(chunk) = stream.try_next().await? {
                let data = if let Some(range) = &range {
                    let start = chunk.start as u64;
                    let end = chunk.end as u64;
                    let offset = start;

                    let start = (start.max(*range.start()) - offset) as usize;
                    let end = (end.min(*range.end()) - offset) as usize;
                    &chunk.data[start..=end]
                } else {
                    &chunk.data
                };

                yield Bytes::copy_from_slice(data)
            }
        }
    }
}

pub struct ChunkMutation;

impl ChunkMutation {
    pub(super) async fn insert(
        db: &impl ConnectionTrait,
        part_id: Uuid,
        index: u64,
        start: u64,
        end: u64,
        data: Vec<u8>,
    ) -> DbRes<InsertResult<chunk::ActiveModel>> {
        let chunk = chunk::ActiveModel {
            id: Set(Uuid::new_v4()),
            part_id: Set(part_id),
            index: Set(index as i64),
            start: Set(start as i64),
            end: Set(end as i64),
            data: Set(data),
            ..Default::default()
        };

        Chunk::insert(chunk).exec(db).await
    }

    pub(super) async fn delete_by_part_id(
        db: &impl ConnectionTrait,
        part_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Chunk::delete_many()
            .filter(chunk::Column::PartId.eq(part_id))
            .exec(db)
            .await
    }
}
