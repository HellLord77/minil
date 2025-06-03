use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BucketCannedAcl {
    Private,
    PublicRead,
    PublicReadWrite,
    AuthenticatedRead,
}
