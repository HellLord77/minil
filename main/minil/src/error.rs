use axum::Extension;
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_s3::error::BucketAlreadyExistsOutput;
use axum_s3::error::BucketAlreadyOwnedByYouOutput;
use axum_s3::error::ConditionalRequestConflictOutput;
use axum_s3::error::EncryptionTypeMismatchOutput;
use axum_s3::error::InvalidWriteOffsetOutput;
use axum_s3::error::NoSuchBucketOutput;
use axum_s3::error::PreconditionFailedOutput;
use axum_s3::error::TooManyPartsOutput;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use sea_orm::DbErr;
use strum::EnumDiscriminants;

use crate::macros::app_err_output;

pub(crate) type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Display, From, Error, EnumDiscriminants)]
pub(crate) enum AppError {
    #[allow(dead_code)]
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    #[allow(dead_code)]
    ConditionalRequestConflict,
    #[allow(dead_code)]
    EncryptionTypeMismatch,
    #[allow(dead_code)]
    InvalidRequest,
    #[allow(dead_code)]
    InvalidWriteOffset,
    NoSuchBucket,
    #[allow(dead_code)]
    PreconditionFailed,
    #[allow(dead_code)]
    TooManyParts,

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
            Self::BucketAlreadyExists => app_err_output!(BucketAlreadyExistsOutput::from(parts)),
            Self::BucketAlreadyOwnedByYou => {
                app_err_output!(BucketAlreadyOwnedByYouOutput::from(parts))
            }
            Self::ConditionalRequestConflict => {
                app_err_output!(ConditionalRequestConflictOutput::from(parts))
            }
            Self::EncryptionTypeMismatch => {
                app_err_output!(EncryptionTypeMismatchOutput::from(parts))
            }
            Self::InvalidRequest => todo!(),
            Self::InvalidWriteOffset => app_err_output!(InvalidWriteOffsetOutput::from(parts)),
            Self::NoSuchBucket => app_err_output!(NoSuchBucketOutput::from(parts)),
            Self::PreconditionFailed => app_err_output!(PreconditionFailedOutput::from(parts)),
            Self::TooManyParts => app_err_output!(TooManyPartsOutput::from(parts)),

            Self::Forbidden => app_err_output!(StatusCode::FORBIDDEN),
            Self::NotImplemented => app_err_output!(StatusCode::NOT_IMPLEMENTED),

            Self::DatabaseError => app_err_output!(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
