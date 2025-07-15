use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_check;

#[validate_check]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListObjectsV2CheckQuery {
    #[validate_check(list_type.contains(&2))]
    pub list_type: Vec<u8>,
}
