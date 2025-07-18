use futures::Stream;
use minil_entity::chunk;
use minil_entity::prelude::*;
use sea_orm::*;
use uuid::Uuid;

use crate::error::DbRes;

pub struct ChunkQuery;

impl ChunkQuery {
    pub async fn find(
        db: &(impl ConnectionTrait + StreamTrait),
        object_id: Option<Uuid>,
        part_id: Option<Uuid>,
        index: u32,
    ) -> DbRes<Option<chunk::Model>> {
        let mut query = Chunk::find();
        if let Some(object_id) = object_id {
            query = query.filter(chunk::Column::ObjectId.eq(object_id));
        }
        if let Some(part_id) = part_id {
            query = query.filter(chunk::Column::PartId.eq(part_id));
        }
        query.filter(chunk::Column::Id.eq(index)).one(db).await
    }

    pub async fn find_all_by_object_id(
        db: &(impl ConnectionTrait + StreamTrait),
        object_id: Uuid,
    ) -> DbRes<impl Stream<Item = DbRes<chunk::Model>>> {
        Chunk::find()
            .filter(chunk::Column::ObjectId.eq(object_id))
            .order_by_asc(chunk::Column::Index)
            .stream(db)
            .await
    }

    pub async fn find_all_by_part_id(
        db: &(impl ConnectionTrait + StreamTrait),
        part: Uuid,
    ) -> DbRes<impl Stream<Item = DbRes<chunk::Model>>> {
        Chunk::find()
            .filter(chunk::Column::PartId.eq(part))
            .order_by_asc(chunk::Column::Index)
            .stream(db)
            .await
    }
}

pub struct ChunkMutation;

impl ChunkMutation {
    pub async fn insert(
        db: &impl ConnectionTrait,
        object_id: Option<Uuid>,
        part_id: Option<Uuid>,
        index: u64,
        data: Vec<u8>,
    ) -> DbRes<InsertResult<chunk::ActiveModel>> {
        let chunk = chunk::ActiveModel {
            id: Set(Uuid::new_v4()),
            object_id: Set(object_id),
            part_id: Set(part_id),
            index: Set(index as i64),
            data: Set(data),
            ..Default::default()
        };

        Chunk::insert(chunk).exec(db).await
    }

    pub async fn delete_all_by_object_id(
        db: &impl ConnectionTrait,
        object_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Chunk::delete_many()
            .filter(chunk::Column::ObjectId.eq(object_id))
            .exec(db)
            .await
    }

    pub async fn delete_all_by_part_id(
        db: &impl ConnectionTrait,
        part_id: Uuid,
    ) -> DbRes<DeleteResult> {
        Chunk::delete_many()
            .filter(chunk::Column::PartId.eq(part_id))
            .exec(db)
            .await
    }
}
