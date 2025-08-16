use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::AbortMultipartUploadInputHeader;
use serde_s3::operation::AbortMultipartUploadInputPath;
use serde_s3::operation::AbortMultipartUploadInputQuery;
use serde_s3::operation::AbortMultipartUploadOutputHeader;

#[derive(Debug, FromRequest)]
pub struct AbortMultipartUploadInput {
    #[from_request(via(Path))]
    pub path: AbortMultipartUploadInputPath,

    #[from_request(via(Query))]
    pub query: AbortMultipartUploadInputQuery,

    #[from_request(via(Header))]
    pub header: AbortMultipartUploadInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct AbortMultipartUploadOutput {
    #[builder(default = StatusCode::NO_CONTENT)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: AbortMultipartUploadOutputHeader,
}
