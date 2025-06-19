use axum::extract::FromRequest;
use axum::extract::Path;
use axum::http::StatusCode;
use axum_header::Header;
use axum_into_response::IntoResponse;
use serde_s3::operation::DeleteBucketInputHeader;
use smart_default::SmartDefault;

#[derive(Debug, FromRequest)]
pub struct DeleteBucketInput {
    #[from_request(via(Path))]
    pub bucket: String,

    #[from_request(via(Header))]
    pub header: DeleteBucketInputHeader,
}

#[derive(Debug, SmartDefault, IntoResponse)]
pub struct DeleteBucketOutput {
    #[default(StatusCode::NO_CONTENT)]
    pub status: StatusCode,
}
