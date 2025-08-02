use axum::extract::FromRequest;
use axum::extract::Path;
use axum_header::Header;
use serde_s3::utils::CommonInputHeader;
use serde_s3::utils::CommonInputPath;

#[derive(Debug, FromRequest)]
pub struct CommonInput {
    #[from_request(via(Path))]
    pub path: CommonInputPath,

    #[from_request(via(Header))]
    pub header: CommonInputHeader,
}
