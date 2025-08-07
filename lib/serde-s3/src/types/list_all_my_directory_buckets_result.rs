use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::utils::Buckets;

#[deprecated]
#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyDirectoryBucketsResult {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    #[builder(into)]
    pub buckets: Option<Buckets>,

    pub continuation_token: Option<String>,
}
