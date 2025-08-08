use futures::Stream;
use futures::TryStreamExt;
use minil_entity::bucket;
use minil_entity::prelude::*;
use minil_entity::tag_set;
use sea_orm::prelude::*;
use sea_orm::*;
use sea_orm_ext::prelude::*;
use sea_query::*;
use uuid::Uuid;

use crate::error::DbRes;

pub struct BucketQuery;

impl BucketQuery {
    pub async fn find(
        db: &impl ConnectionTrait,
        owner_id: Uuid,
        name: &str,
    ) -> DbRes<Option<bucket::Model>> {
        Bucket::find()
            .filter(bucket::Column::OwnerId.eq(owner_id))
            .filter(bucket::Column::Name.eq(name))
            .one(db)
            .await
    }

    pub async fn find_also_tag_set(
        db: &impl ConnectionTrait,
        owner_id: Uuid,
        name: &str,
    ) -> DbRes<Option<(bucket::Model, Option<tag_set::Model>)>> {
        Bucket::find()
            .find_also_related(TagSet)
            .filter(bucket::Column::OwnerId.eq(owner_id))
            .filter(bucket::Column::Name.eq(name))
            .one(db)
            .await
    }

    pub async fn find_many(
        db: &(impl ConnectionTrait + StreamTrait),
        owner_id: Uuid,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
        limit: Option<u64>,
    ) -> DbRes<impl Stream<Item = DbRes<bucket::Model>>> {
        let mut query = Bucket::find().filter(bucket::Column::OwnerId.eq(owner_id));
        if let Some(prefix) = prefix {
            query = query.filter(bucket::Column::Name.starts_with(prefix));
        }
        if let Some(continuation_token) = continuation_token {
            query = query.filter(bucket::Column::Name.gt(continuation_token));
        }
        query
            .order_by_asc(bucket::Column::Name)
            .limit(limit)
            .stream(db)
            .await
    }
}

pub struct BucketMutation;

impl BucketMutation {
    pub async fn insert(
        db: &impl ConnectionTrait,
        owner_id: Uuid,
        name: String,
    ) -> DbRes<Option<bucket::Model>> {
        let bucket = bucket::ActiveModel {
            id: Set(Uuid::new_v4()),
            owner_id: Set(owner_id),
            name: Set(name),
            ..Default::default()
        };

        Bucket::insert(bucket)
            .on_conflict(
                OnConflict::columns([bucket::Column::OwnerId, bucket::Column::Name])
                    .value(bucket::Column::UpdatedAt, Expr::current_timestamp())
                    .to_owned(),
            )
            .exec_with_returning(db)
            .await
            .map(Some)
            .or_else(|err| match err {
                DbErr::RecordNotFound(_) => Ok(None),
                _ => Err(err),
            })
    }

    pub async fn update_versioning(
        db: &(impl ConnectionTrait + StreamTrait),
        owner_id: Uuid,
        name: &str,
        mfa_delete: Option<bool>,
        versioning: Option<bool>,
    ) -> DbRes<Option<bucket::Model>> {
        let bucket = bucket::ActiveModel {
            mfa_delete: mfa_delete.map(Set).unwrap_or_default().into(),
            versioning: versioning.map(Set).unwrap_or_default().into(),
            ..Default::default()
        };

        Bucket::update_many()
            .filter(bucket::Column::OwnerId.eq(owner_id))
            .filter(bucket::Column::Name.eq(name))
            .set(bucket)
            .col_expr(bucket::Column::UpdatedAt, Expr::current_timestamp().into())
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }

    pub async fn delete(
        db: &(impl ConnectionTrait + StreamTrait),
        owner_id: Uuid,
        name: &str,
    ) -> DbRes<Option<bucket::Model>> {
        Bucket::delete_many()
            .filter(bucket::Column::OwnerId.eq(owner_id))
            .filter(bucket::Column::Name.eq(name))
            .exec_with_streaming(db)
            .await?
            .try_next()
            .await
    }
}
