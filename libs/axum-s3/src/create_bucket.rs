use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_xml::Xml;
use derive_getters::Getters;
use serde_s3::operation::CreateBucketInputBody;
use serde_s3::operation::CreateBucketInputHeader;
use serde_s3::operation::CreateBucketOutputHeader;

#[derive(Debug, Getters, FromRequest)]
pub struct CreateBucketInput {
    #[from_request(via(Path))]
    bucket: String,

    #[from_request(via(Header))]
    header: CreateBucketInputHeader,

    #[from_request(via(Xml))]
    body: CreateBucketInputBody,
}

#[derive(Debug, IntoResponse)]
pub struct CreateBucketOutput {
    #[into_response(via(Header))]
    pub header: CreateBucketOutputHeader,
}
