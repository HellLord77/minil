use std::io;

use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use sea_orm::DbErr;

pub(super) type DbRes<T> = Result<T, DbErr>;

pub type InsRes<T> = Result<T, InsErr>;

#[derive(Debug, Display, From, Error)]
pub enum InsErr {
    IoError(io::Error),
    DbError(DbErr),
}
