use std::marker::PhantomData;

use minil_entity::bucket;
use minil_entity::prelude::*;
use sea_orm::*;
use uuid::Uuid;

use crate::error::DbRes;

pub struct BucketQuery<C>(PhantomData<C>);

impl<C> BucketQuery<C>
where
    C: ConnectionTrait,
{
    pub async fn find_by_unique_id(
        db: &C,
        owner_id: Uuid,
        name: &str,
    ) -> DbRes<Option<bucket::Model>> {
        Bucket::find()
            .filter(bucket::Column::OwnerId.eq(owner_id))
            .filter(bucket::Column::Name.eq(name))
            .one(db)
            .await
    }

    pub async fn find_all_by_owner_id(
        db: &C,
        owner_id: Uuid,
        starts_with: Option<&str>,
        start_after: Option<&str>,
        limit: u16,
    ) -> DbRes<Vec<bucket::Model>> {
        let mut query = Bucket::find().filter(bucket::Column::OwnerId.eq(owner_id));
        if let Some(starts_with) = starts_with {
            query = query.filter(bucket::Column::Name.starts_with(starts_with));
        }
        if let Some(start_after) = start_after {
            query = query.filter(bucket::Column::Name.gte(start_after));
        }
        query
            .order_by_asc(bucket::Column::Name)
            .limit(Some(limit as u64))
            .all(db)
            .await
    }
}

pub struct BucketMutation<C>(PhantomData<C>);

impl<C> BucketMutation<C>
where
    C: ConnectionTrait,
{
    async fn insert(db: &C, bucket: bucket::ActiveModel) -> DbRes<Option<bucket::Model>> {
        TryInsert::one(bucket)
            .on_conflict(
                sea_query::OnConflict::columns([bucket::Column::OwnerId, bucket::Column::Name])
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
        db: &C,
        owner_id: Uuid,
        name: &str,
        region: &str,
    ) -> DbRes<Option<bucket::Model>> {
        let bucket = bucket::ActiveModel {
            id: Set(Uuid::new_v4()),
            owner_id: Set(owner_id),
            name: Set(name.to_owned()),
            region: Set(region.to_owned()),
            ..Default::default()
        };
        BucketMutation::insert(db, bucket).await
    }

    async fn delete(db: &C, bucket: bucket::Model) -> DbRes<Option<bucket::Model>> {
        Delete::one(bucket).exec_with_returning(db).await
    }

    pub async fn delete_by_unique_id(
        db: &C,
        owner_id: Uuid,
        name: &str,
    ) -> DbRes<Option<bucket::Model>> {
        let bucket = BucketQuery::find_by_unique_id(db, owner_id, name).await?;
        match bucket {
            Some(bucket) => BucketMutation::delete(db, bucket).await,
            None => Ok(None),
        }
    }
}
