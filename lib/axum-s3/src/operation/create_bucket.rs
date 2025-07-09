use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::CreateBucketInputBody;
use serde_s3::operation::CreateBucketInputHeader;
use serde_s3::operation::CreateBucketInputPath;
use serde_s3::operation::CreateBucketOutputHeader;

#[derive(Debug, FromRequest)]
pub struct CreateBucketInput {
    #[from_request(via(Path))]
    pub path: CreateBucketInputPath,

    #[from_request(via(Header))]
    pub header: CreateBucketInputHeader,

    #[from_request(via(Xml))]
    pub body: Option<CreateBucketInputBody>,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct CreateBucketOutput {
    #[into_response(via(Header))]
    pub header: CreateBucketOutputHeader,
}
