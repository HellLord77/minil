use bon::Builder;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use serdev::Serialize;
use validator::Validate;
use validator_extra::validate_extra;

use crate::utils::TagSet;

#[validate_extra]
#[skip_serializing_none]
#[derive(Debug, Builder, Validate, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct Tagging {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/".to_owned())]
    #[validate_extra(eq(other = "http://s3.amazonaws.com/doc/2006-03-01/"))]
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[builder(into)]
    pub tag_set: TagSet,
}
