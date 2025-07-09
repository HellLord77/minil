use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::GetBucketLocationInputHeader;
use serde_s3::operation::GetBucketLocationInputPath;
use serde_s3::operation::GetBucketLocationOutputBody;

#[derive(Debug, FromRequest)]
pub struct GetBucketLocationInput {
    #[from_request(via(Path))]
    pub path: GetBucketLocationInputPath,

    #[from_request(via(Header))]
    pub header: GetBucketLocationInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct GetBucketLocationOutput {
    #[into_response(via(Xml))]
    pub body: GetBucketLocationOutputBody,
}
