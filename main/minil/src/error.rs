use axum::Extension;
use axum::response::IntoResponse;
use axum::response::Response;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use sea_orm::DbErr;
use strum::EnumDiscriminants;

#[derive(Debug, Display, From, Error, EnumDiscriminants)]
pub(crate) enum AppError {
    #[allow(dead_code)]
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    NoSuchBucket,

    Forbidden,
    NotImplemented,

    DbErr(DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[allow(clippy::single_match)]
        match &self {
            Self::DbErr(err) => {
                tracing::error!(%err, "DbErr");
            }
            _ => {}
        };

        Extension(AppErrorDiscriminants::from(self)).into_response()
    }
}

pub(crate) type AppResult<T> = Result<T, AppError>;
