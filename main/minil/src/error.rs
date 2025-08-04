use std::io;

use axum::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_s3::utils::CommonExtInput;
use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use minil_service::InsErr;
use sea_orm::DbErr;
use strum::EnumDiscriminants;

use crate::macros::app_log_err;
use crate::macros::app_response_err;

pub(crate) type AppResult<T> = Result<T, AppError>;

// todo macro
#[derive(Debug, Display, From, Error, EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
pub(crate) enum AppError {
    AccessDenied,
    #[allow(dead_code)]
    BadDigest,
    #[allow(dead_code)]
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    #[allow(dead_code)]
    ConditionalRequestConflict,
    #[allow(dead_code)]
    EncryptionTypeMismatch,
    #[allow(dead_code)]
    IncompleteBody,
    #[deprecated]
    InternalError,
    #[allow(dead_code)]
    InvalidDigest,
    #[allow(dead_code)]
    InvalidObjectState,
    InvalidPart,
    #[allow(dead_code)]
    InvalidPartOrder,
    InvalidRange,
    #[allow(dead_code)]
    InvalidTag,
    #[allow(dead_code)]
    InvalidWriteOffset,
    #[allow(dead_code)]
    MalformedXML,
    MethodNotAllowed,
    NoSuchBucket,
    NoSuchKey,
    NoSuchTagSet,
    #[allow(dead_code)]
    NoSuchUpload,
    NoSuchVersion,
    NotImplemented,
    #[allow(dead_code)]
    OperationAborted,
    #[allow(dead_code)]
    PreconditionFailed,
    #[allow(dead_code)]
    TooManyParts,

    AxumError(axum::Error),
    DatabaseError(DbErr),
    IoError(io::Error),
}

impl From<InsErr> for AppError {
    fn from(err: InsErr) -> Self {
        match err {
            InsErr::IoError(err) => Self::IoError(err),
            InsErr::DbError(err) => Self::DatabaseError(err),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        app_log_err!(&self => [AxumError, DatabaseError, IoError]);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Extension(AppErrorDiscriminants::from(&self)),
        )
            .into_response()
    }
}

impl AppErrorDiscriminants {
    #[inline]
    pub(crate) fn into_response(self, common: CommonExtInput) -> Response {
        app_response_err!((self, common) {
            AccessDenied => AccessDeniedOutput,
            BadDigest => BadDigestOutput,
            BucketAlreadyExists => BucketAlreadyExistsOutput,
            BucketAlreadyOwnedByYou => BucketAlreadyOwnedByYouOutput,
            ConditionalRequestConflict => ConditionalRequestConflictOutput,
            EncryptionTypeMismatch => EncryptionTypeMismatchOutput,
            IncompleteBody => IncompleteBodyOutput,
            InternalError => InternalErrorOutput,
            InvalidDigest => InvalidDigestOutput,
            InvalidObjectState => InvalidObjectStateOutput,
            InvalidPart => InvalidPartOutput,
            InvalidPartOrder => InvalidPartOrderOutput,
            InvalidTag => InvalidTagOutput,
            InvalidRange => InvalidRangeOutput,
            InvalidWriteOffset => InvalidWriteOffsetOutput,
            MalformedXML => MalformedXMLOutput,
            MethodNotAllowed => MethodNotAllowedOutput,
            NoSuchBucket => NoSuchBucketOutput,
            NoSuchKey => NoSuchKeyOutput,
            NoSuchTagSet => NoSuchTagSetOutput,
            NoSuchUpload => NoSuchUploadOutput,
            NotImplemented => NotImplementedOutput,
            NoSuchVersion => NoSuchVersionOutput,
            OperationAborted => OperationAbortedOutput,
            PreconditionFailed => PreconditionFailedOutput,
            TooManyParts => TooManyPartsOutput,
            _ => [AxumError, DatabaseError, IoError],
        })
    }
}
