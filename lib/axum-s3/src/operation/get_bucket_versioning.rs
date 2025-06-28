use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use axum_into_response::IntoResponse;
use axum_xml::Xml;
use bon::Builder;
use serde_s3::operation::GetBucketVersioningInputHeader;
use serde_s3::operation::GetBucketVersioningInputPath;
use serde_s3::operation::GetBucketVersioningOutputBody;

#[derive(Debug, FromRequest)]
pub struct GetBucketVersioningInput {
    #[from_request(via(Path))]
    pub path: GetBucketVersioningInputPath,

    #[from_request(via(Header))]
    pub header: GetBucketVersioningInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct GetBucketVersioningOutput {
    #[into_response(via(Xml))]
    pub body: GetBucketVersioningOutputBody,
}
