use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use sea_orm::*;

#[async_trait]
pub trait SelectTwoExt<E, F>
where
    E: EntityTrait,
    F: EntityTrait,
{
    async fn one_both<C>(self, db: &C) -> Result<Option<(E::Model, F::Model)>, DbErr>
    where
        C: ConnectionTrait;

    async fn all_both<C>(self, db: &C) -> Result<Vec<(E::Model, F::Model)>, DbErr>
    where
        C: ConnectionTrait;

    async fn stream_both<'a: 'b, 'b, C>(
        self,
        db: &'a C,
    ) -> Result<impl Stream<Item = Result<(E::Model, F::Model), DbErr>> + 'b, DbErr>
    where
        C: ConnectionTrait + StreamTrait + Send;
}

#[async_trait]
impl<E, F> SelectTwoExt<E, F> for SelectTwo<E, F>
where
    E: EntityTrait,
    F: EntityTrait,
{
    async fn one_both<C>(self, db: &C) -> Result<Option<(E::Model, F::Model)>, DbErr>
    where
        C: ConnectionTrait,
    {
        self.one(db)
            .await
            .map(|res| res.map(|(model1, model2)| (model1, model2.unwrap())))
    }

    async fn all_both<C>(self, db: &C) -> Result<Vec<(E::Model, F::Model)>, DbErr>
    where
        C: ConnectionTrait,
    {
        self.all(db).await.map(|res| {
            res.into_iter()
                .map(|(model1, model2)| (model1, model2.unwrap()))
                .collect()
        })
    }

    async fn stream_both<'a: 'b, 'b, C>(
        self,
        db: &'a C,
    ) -> Result<impl Stream<Item = Result<(E::Model, F::Model), DbErr>> + 'b, DbErr>
    where
        C: ConnectionTrait + StreamTrait + Send,
    {
        Ok(self
            .stream(db)
            .await?
            .map(|res| res.map(|(model1, model2)| (model1, model2.unwrap()))))
    }
}
