use bon::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::Buckets;
use crate::types::Owner;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListAllMyBucketsResult {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    #[builder(into)]
    pub buckets: Option<Buckets>,

    pub owner: Option<Owner>,

    pub continuation_token: Option<String>,

    pub prefix: Option<String>,
}
