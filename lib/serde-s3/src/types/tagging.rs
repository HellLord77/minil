use bon::Builder;
use serde::Deserialize;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::utils::TagSet;

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tagging {
    #[builder(into)]
    pub tag_set: TagSet,
}
