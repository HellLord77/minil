use axum::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_s3::error::AccessDeniedOutput;
use axum_s3::error::BadDigestOutput;
use axum_s3::error::BucketAlreadyExistsOutput;
use axum_s3::error::BucketAlreadyOwnedByYouOutput;
use axum_s3::error::ConditionalRequestConflictOutput;
use axum_s3::error::EncryptionTypeMismatchOutput;
use axum_s3::error::InternalErrorOutput;
use axum_s3::error::InvalidDigestOutput;
use axum_s3::error::InvalidWriteOffsetOutput;
use axum_s3::error::NoSuchBucketOutput;
use axum_s3::error::NoSuchKeyOutput;
use axum_s3::error::NoSuchUploadOutput;
use axum_s3::error::NotImplementedOutput;
use axum_s3::error::PreconditionFailedOutput;
use axum_s3::error::TooManyPartsOutput;
use axum_s3::utils::ErrorParts;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use sea_orm::DbErr;
use strum::EnumDiscriminants;

use crate::macros::app_err_output;
use crate::macros::app_log_err;

pub(crate) type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Display, From, Error, EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
pub(crate) enum AppError {
    #[allow(dead_code)]
    AccessDenied,
    BadDigest,
    #[allow(dead_code)]
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    #[allow(dead_code)]
    ConditionalRequestConflict,
    #[allow(dead_code)]
    EncryptionTypeMismatch,
    #[allow(dead_code)]
    InternalError,
    InvalidDigest,
    #[allow(dead_code)]
    InvalidWriteOffset,
    NoSuchBucket,
    #[allow(dead_code)]
    NoSuchKey,
    #[allow(dead_code)]
    NoSuchUpload,
    NotImplemented,
    #[allow(dead_code)]
    PreconditionFailed,
    #[allow(dead_code)]
    TooManyParts,

    AxumError(axum::Error),
    DatabaseError(DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        app_log_err!(&self => AxumError, DatabaseError);
        Extension(AppErrorDiscriminants::from(&self)).into_response()
    }
}

impl AppErrorDiscriminants {
    #[inline]
    pub(crate) fn into_response(self, parts: ErrorParts) -> Response {
        match self {
            Self::AccessDenied => app_err_output!(AccessDeniedOutput::from(parts)),
            Self::BadDigest => app_err_output!(BadDigestOutput::from(parts)),
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
            Self::InternalError => app_err_output!(InternalErrorOutput::from(parts)),
            Self::InvalidWriteOffset => app_err_output!(InvalidWriteOffsetOutput::from(parts)),
            Self::InvalidDigest => app_err_output!(InvalidDigestOutput::from(parts)),
            Self::NoSuchBucket => app_err_output!(NoSuchBucketOutput::from(parts)),
            Self::NoSuchKey => app_err_output!(NoSuchKeyOutput::from(parts)),
            Self::NoSuchUpload => app_err_output!(NoSuchUploadOutput::from(parts)),
            Self::NotImplemented => app_err_output!(NotImplementedOutput::from(parts)),
            Self::PreconditionFailed => app_err_output!(PreconditionFailedOutput::from(parts)),
            Self::TooManyParts => app_err_output!(TooManyPartsOutput::from(parts)),

            Self::AxumError => app_err_output!(StatusCode::INTERNAL_SERVER_ERROR),
            Self::DatabaseError => app_err_output!(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
