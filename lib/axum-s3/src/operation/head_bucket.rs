use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use axum_into_response::IntoResponse;
use bon::Builder;
use serde_s3::operation::DeleteBucketInputHeader;
use serde_s3::operation::HeadBucketOutputHeader;

#[derive(Debug, FromRequest)]
pub struct HeadBucketInput {
    #[from_request(via(Path))]
    pub bucket: String,

    #[from_request(via(Header))]
    pub header: DeleteBucketInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct HeadBucketOutput {
    #[into_response(via(Header))]
    pub header: HeadBucketOutputHeader,
}
