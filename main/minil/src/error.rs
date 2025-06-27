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
    #[allow(dead_code)]
    ConditionalRequestConflict,
    NoSuchBucket,
    #[allow(dead_code)]
    PreconditionFailed,

    Forbidden,
    NotImplemented,

    DatabaseError(DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[allow(clippy::single_match)]
        match &self {
            Self::DatabaseError(err) => {
                tracing::error!(%err, "DatabaseError");
            }
            _ => {}
        };

        Extension(AppErrorDiscriminants::from(self)).into_response()
    }
}

impl AppErrorDiscriminants {
    pub(crate) fn into_response(self, parts: &Parts) -> Response {
        match self {
            Self::BucketAlreadyExists => {
                let output = BucketAlreadyExistsOutput::from(parts);

                dbg!(&output);
                output.into_response()
            }
            Self::BucketAlreadyOwnedByYou => {
                let output = BucketAlreadyOwnedByYouOutput::from(parts);

                dbg!(&output);
                output.into_response()
            }
            Self::ConditionalRequestConflict => {
                let output = axum_s3::error::ConditionalRequestConflictOutput::from(parts);

                dbg!(&output);
                output.into_response()
            }
            Self::NoSuchBucket => {
                let output = NoSuchBucketOutput::from(parts);

                dbg!(&output);
                output.into_response()
            }
            Self::PreconditionFailed => {
                let output = axum_s3::error::PreconditionFailedOutput::from(parts);

                dbg!(&output);
                output.into_response()
            }

            Self::Forbidden => {
                let output = StatusCode::FORBIDDEN;

                dbg!(&output);
                output.into_response()
            }
            Self::NotImplemented => {
                let output = StatusCode::NOT_IMPLEMENTED;

                dbg!(&output);
                output.into_response()
            }

            Self::DatabaseError => {
                let output = StatusCode::INTERNAL_SERVER_ERROR;

                dbg!(&output);
                output.into_response()
            }
        }
    }
}
