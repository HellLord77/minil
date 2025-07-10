use axum::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_s3::utils::ErrorParts;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use sea_orm::DbErr;
use strum::EnumDiscriminants;

use crate::macros::app_log_err;
use crate::macros::app_response_err;

pub(crate) type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Display, From, Error, EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
pub(crate) enum AppError {
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
        app_log_err!(&self => [AxumError, DatabaseError]);
        Extension(AppErrorDiscriminants::from(&self)).into_response()
    }
}

impl AppErrorDiscriminants {
    #[inline]
    pub(crate) fn into_response(self, parts: ErrorParts) -> Response {
        app_response_err!((self, parts) {
            AccessDenied => AccessDeniedOutput,
            BadDigest => BadDigestOutput,
            BucketAlreadyExists => BucketAlreadyExistsOutput,
            BucketAlreadyOwnedByYou => BucketAlreadyOwnedByYouOutput,
            ConditionalRequestConflict => ConditionalRequestConflictOutput,
            EncryptionTypeMismatch => EncryptionTypeMismatchOutput,
            InternalError => InternalErrorOutput,
            InvalidDigest => InvalidDigestOutput,
            InvalidWriteOffset => InvalidWriteOffsetOutput,
            NoSuchBucket => NoSuchBucketOutput,
            NoSuchKey => NoSuchKeyOutput,
            NoSuchUpload => NoSuchUploadOutput,
            NotImplemented => NotImplementedOutput,
            PreconditionFailed => PreconditionFailedOutput,
            TooManyParts => TooManyPartsOutput,
            _ => !
            AxumError => INTERNAL_SERVER_ERROR,
            DatabaseError => INTERNAL_SERVER_ERROR,
        })
    }
}
