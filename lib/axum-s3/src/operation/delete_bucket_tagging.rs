use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::DeleteBucketTaggingInputHeader;
use serde_s3::operation::DeleteBucketTaggingInputPath;

#[derive(Debug, FromRequest)]
pub struct DeleteBucketTaggingInput {
    #[from_request(via(Path))]
    pub path: DeleteBucketTaggingInputPath,

    #[from_request(via(Header))]
    pub header: DeleteBucketTaggingInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct DeleteBucketTaggingOutput {
    #[builder(default = StatusCode::NO_CONTENT)]
    pub status: StatusCode,
}
