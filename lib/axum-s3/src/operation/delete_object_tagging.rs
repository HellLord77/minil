use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::DeleteObjectTaggingInputHeader;
use serde_s3::operation::DeleteObjectTaggingInputPath;
use serde_s3::operation::DeleteObjectTaggingInputQuery;
use serde_s3::operation::DeleteObjectTaggingOutputHeader;

#[derive(Debug, FromRequest)]
pub struct DeleteObjectTaggingInput {
    #[from_request(via(Path))]
    pub path: DeleteObjectTaggingInputPath,

    #[from_request(via(Query))]
    pub query: DeleteObjectTaggingInputQuery,

    #[from_request(via(Header))]
    pub header: DeleteObjectTaggingInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct DeleteObjectTaggingOutput {
    #[builder(default = StatusCode::OK)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: DeleteObjectTaggingOutputHeader,
}
