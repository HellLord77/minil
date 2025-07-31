use axum::extract::FromRequestParts;
use axum::http::Uri;
use axum_header::Header;
use serde_s3::utils::CommonExtInputHeader;

#[derive(Debug, FromRequestParts)]
pub struct CommonExtInput {
    pub path: Uri,

    #[from_request(via(Header))]
    pub header: CommonExtInputHeader,
}
