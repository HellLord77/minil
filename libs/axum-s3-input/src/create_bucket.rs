use axum::extract::FromRequest;
use axum_header::Header;
use axum_xml::Xml;
use serde_s3::operation::CreateBucketInputBody;
use serde_s3::operation::CreateBucketInputHeader;

#[derive(Debug, FromRequest)]
pub struct CreateBucketInput {
    #[from_request(via(Header))]
    pub header: CreateBucketInputHeader,

    #[from_request(via(Xml))]
    pub body: CreateBucketInputBody,
}
