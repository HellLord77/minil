use axum::extract::Path;
use axum_body::OptionalEmpty;
use axum_derive_macros::IntoResponse;
use axum_derive_macros::from_request_optional;
use axum_header::Header;
use axum_serde::Xml;
use bon::Builder;
use serde_s3::operation::CreateBucketInputBody;
use serde_s3::operation::CreateBucketInputHeader;
use serde_s3::operation::CreateBucketInputPath;
use serde_s3::operation::CreateBucketOutputHeader;

#[from_request_optional]
#[derive(Debug)]
pub struct CreateBucketInput {
    #[from_request(via(Path))]
    pub path: CreateBucketInputPath,

    #[from_request(via(Header))]
    pub header: CreateBucketInputHeader,

    #[from_request(via(Xml))]
    #[from_request_optional(via(OptionalEmpty))]
    pub body: Option<CreateBucketInputBody>,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct CreateBucketOutput {
    #[into_response(via(Header))]
    pub header: CreateBucketOutputHeader,
}
