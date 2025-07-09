use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::DeleteBucketInputHeader;
use serde_s3::operation::DeleteBucketInputPath;

#[derive(Debug, FromRequest)]
pub struct DeleteBucketInput {
    #[from_request(via(Path))]
    pub path: DeleteBucketInputPath,

    #[from_request(via(Header))]
    pub header: DeleteBucketInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct DeleteBucketOutput {
    #[builder(default = StatusCode::NO_CONTENT)]
    pub status: StatusCode,
}
