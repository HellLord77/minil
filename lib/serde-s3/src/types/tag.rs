use bon::Builder;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use serdev::Serialize;
use validator::Validate;

#[skip_serializing_none]
#[derive(Debug, Builder, Validate, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct Tag {
    #[validate(length(min = 1, max = 128))]
    pub key: String,

    #[validate(length(max = 256))]
    pub value: String,
}
