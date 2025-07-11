use sea_orm::DbErr;

pub(super) type DbRes<T> = Result<T, DbErr>;
