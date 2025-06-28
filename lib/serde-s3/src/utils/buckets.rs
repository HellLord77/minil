use derive_more::From;
use serde::Serialize;

use crate::types::Bucket;

#[derive(Debug, From, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Buckets {
    bucket: Vec<Bucket>,
}
