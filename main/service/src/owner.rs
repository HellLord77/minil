use minil_entity::owner;
use minil_entity::prelude::*;
use sea_orm::*;

use crate::error::DbRes;

pub struct OwnerQuery;

impl OwnerQuery {
    pub async fn find(db: &impl ConnectionTrait, name: &str) -> DbRes<Option<owner::Model>> {
        Owner::find()
            .filter(owner::Column::Name.eq(name))
            .one(db)
            .await
    }
}

pub struct OwnerMutation;

impl OwnerMutation {}
