use ::entity::owner;
use ::entity::prelude::*;
use sea_orm::*;
use uuid::Uuid;

pub struct Query;

impl Query {
    pub async fn find_by_id(db: &DbConn, id: Uuid) -> Result<Option<owner::Model>, DbErr> {
        Owner::find_by_id(id).one(db).await
    }

    pub async fn find_by_name(db: &DbConn, name: &str) -> Result<Option<owner::Model>, DbErr> {
        Owner::find()
            .filter(owner::Column::Name.eq(name))
            .one(db)
            .await
    }
}
