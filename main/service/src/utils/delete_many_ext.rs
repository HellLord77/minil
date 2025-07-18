use futures::Stream;
use minil_entity::object;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DbErr;
use sea_orm::DeleteMany;
use sea_orm::EntityTrait;
use sea_orm::Iterable;
use sea_orm::QueryTrait;
use sea_orm::SelectModel;
use sea_orm::SelectorRaw;
use sea_orm::StreamTrait;
use sea_orm::sea_query::DeleteStatement;
use sea_orm::sea_query::Query;

pub(crate) trait DeleteManyExt<E>
where
    E: EntityTrait,
{
    fn exec_with_streaming<C>(
        self,
        db: &C,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<object::Model, DbErr>>, DbErr>> + '_
    where
        C: ConnectionTrait + StreamTrait;
}

impl<E> DeleteManyExt<E> for DeleteMany<E>
where
    E: EntityTrait,
{
    fn exec_with_streaming<C>(
        self,
        db: &C,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<object::Model, DbErr>>, DbErr>> + '_
    where
        C: ConnectionTrait + StreamTrait,
    {
        exec_delete_with_streaming::<E, _>(self.into_query(), db)
    }
}

async fn exec_delete_with_streaming<E, C>(
    mut query: DeleteStatement,
    db: &C,
) -> Result<impl Stream<Item = Result<object::Model, DbErr>>, DbErr>
where
    E: EntityTrait,
    C: ConnectionTrait + StreamTrait,
{
    let models = match db.support_returning() {
        true => {
            let db_backend = db.get_database_backend();
            let returning = Query::returning().exprs(
                E::Column::iter().map(|c| c.select_enum_as(c.into_returning_expr(db_backend))),
            );
            let query = query.returning(returning);
            let delete_statement = db_backend.build(&query.to_owned());
            SelectorRaw::<SelectModel<<E>::Model>>::from_statement(delete_statement)
                .stream(db)
                .await?
        }
        false => unimplemented!("Database backend doesn't support RETURNING"),
    };
    Ok(models)
}
