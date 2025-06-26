use axum::Extension;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_s3::error::BucketAlreadyExistsOutput;
use axum_s3::error::BucketAlreadyOwnedByYouOutput;
use axum_s3::error::NoSuchBucketOutput;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use sea_orm::DbErr;
use strum::EnumDiscriminants;

pub(crate) type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Display, From, Error, EnumDiscriminants)]
pub(crate) enum AppError {
    #[allow(dead_code)]
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    NoSuchBucket,

    Forbidden,
    NotImplemented,

    DatabaseError(DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[allow(clippy::single_match)]
        match &self {
            Self::DatabaseError(err) => {
                tracing::error!(%err, "DbErr");
            }
            _ => {}
        };

        Extension(AppErrorDiscriminants::from(self)).into_response()
    }
}

impl AppErrorDiscriminants {
    pub(crate) fn into_response(self, parts: &Parts) -> Response {
        match self {
            AppErrorDiscriminants::BucketAlreadyExists => {
                BucketAlreadyExistsOutput::from(parts).into_response()
            }
            AppErrorDiscriminants::BucketAlreadyOwnedByYou => {
                BucketAlreadyOwnedByYouOutput::from(parts).into_response()
            }
            AppErrorDiscriminants::NoSuchBucket => NoSuchBucketOutput::from(parts).into_response(),

            AppErrorDiscriminants::Forbidden => StatusCode::FORBIDDEN.into_response(),
            AppErrorDiscriminants::NotImplemented => StatusCode::NOT_IMPLEMENTED.into_response(),

            AppErrorDiscriminants::DatabaseError => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
