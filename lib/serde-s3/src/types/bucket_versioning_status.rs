use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum BucketVersioningStatus {
    Enabled,
    Suspended,
}
