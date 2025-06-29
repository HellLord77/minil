use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ObjectCannedAcl {
    AuthenticatedRead,
    AwsExecRead,
    BucketOwnerFullControl,
    BucketOwnerRead,
    Private,
    PublicRead,
    PublicReadWrite,
}
