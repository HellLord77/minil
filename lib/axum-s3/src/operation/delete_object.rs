use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_derive_macros::IntoResponse;
use axum_extra::extract::Query;
use axum_header::Header;
use bon::Builder;
use serde_s3::operation::DeleteObjectInputHeader;
use serde_s3::operation::DeleteObjectInputPath;
use serde_s3::operation::DeleteObjectInputQuery;
use serde_s3::operation::DeleteObjectOutputHeader;

#[derive(Debug, FromRequest)]
pub struct DeleteObjectInput {
    #[from_request(via(Path))]
    pub path: DeleteObjectInputPath,

    #[from_request(via(Query))]
    pub query: DeleteObjectInputQuery,

    #[from_request(via(Header))]
    pub header: DeleteObjectInputHeader,
}

#[derive(Debug, Builder, IntoResponse)]
pub struct DeleteObjectOutput {
    #[builder(default = StatusCode::NO_CONTENT)]
    pub status: StatusCode,

    #[into_response(via(Header))]
    pub header: DeleteObjectOutputHeader,
}
