use std::marker::PhantomData;

use minil_entity::owner;
use minil_entity::prelude::*;
use sea_orm::*;

use crate::error::DbRes;

pub struct OwnerQuery<C>(PhantomData<C>);

impl<C> OwnerQuery<C>
where
    C: ConnectionTrait,
{
    pub async fn find_by_unique_id(db: &C, name: &str) -> DbRes<Option<owner::Model>> {
        Owner::find()
            .filter(owner::Column::Name.eq(name))
            .one(db)
            .await
    }
}

pub struct OwnerMutation<C>(PhantomData<C>);

impl<C> OwnerMutation<C> where C: ConnectionTrait {}
