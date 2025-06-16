use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BucketCannedAcl {
    Private,
    PublicRead,
    PublicReadWrite,
    AuthenticatedRead,
}
