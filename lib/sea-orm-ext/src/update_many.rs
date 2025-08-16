use futures::Stream;
use futures::StreamExt;
use futures::stream;
use sea_orm::*;
use sea_query::*;

pub trait UpdateManyExt<E>
where
    E: EntityTrait,
{
    fn exec_with_streaming<C>(
        self,
        db: &C,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<E::Model, DbErr>>, DbErr>> + '_
    where
        C: ConnectionTrait + StreamTrait;
}

impl<E> UpdateManyExt<E> for UpdateMany<E>
where
    E: EntityTrait,
{
    fn exec_with_streaming<C>(
        self,
        db: &C,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<E::Model, DbErr>>, DbErr>> + '_
    where
        C: ConnectionTrait + StreamTrait,
    {
        exec_update_with_streaming::<E, _>(self.into_query(), db)
    }
}

async fn exec_update_with_streaming<E, C>(
    mut query: UpdateStatement,
    db: &C,
) -> Result<impl Stream<Item = Result<E::Model, DbErr>>, DbErr>
where
    E: EntityTrait,
    C: ConnectionTrait + StreamTrait,
{
    if is_noop(&query) {
        return Ok(stream::empty().left_stream());
    }

    if db.support_returning() {
        let db_backend = db.get_database_backend();
        let returning = Query::returning()
            .exprs(E::Column::iter().map(|c| c.select_as(c.into_returning_expr(db_backend))));
        query.returning(returning);
        let models = SelectorRaw::<SelectModel<E::Model>>::from_statement(db_backend.build(&query))
            .stream(db)
            .await?;
        Ok(models.right_stream())
    } else {
        unimplemented!("Database backend doesn't support RETURNING")
    }
}

fn is_noop(query: &UpdateStatement) -> bool {
    query.get_values().is_empty()
}
