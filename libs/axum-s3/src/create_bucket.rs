use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_xml::Xml;
use serde_s3::operation::CreateBucketInputBody;
use serde_s3::operation::CreateBucketInputHeader;
use serde_s3::operation::CreateBucketOutputHeader;

#[derive(Debug, FromRequest)]
pub struct CreateBucketInput {
    #[from_request(via(Path))]
    pub bucket: String,

    #[from_request(via(Header))]
    pub header: CreateBucketInputHeader,

    #[from_request(via(Xml))]
    pub body: CreateBucketInputBody,
}

#[derive(Debug, IntoResponse)]
pub struct CreateBucketOutput {
    #[into_response(via(Header))]
    pub header: CreateBucketOutputHeader,
}
