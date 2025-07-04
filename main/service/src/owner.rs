use minil_entity::owner;
use minil_entity::prelude::*;
use sea_orm::*;

pub struct OwnerQuery;

impl OwnerQuery {
    pub async fn find_by_unique_id(db: &DbConn, name: &str) -> Result<Option<owner::Model>, DbErr> {
        Owner::find()
            .filter(owner::Column::Name.eq(name))
            .one(db)
            .await
    }
}

pub struct OwnerMutation;

impl OwnerMutation {}
