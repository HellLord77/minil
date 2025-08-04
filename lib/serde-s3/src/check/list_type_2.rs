use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

#[validate_extra]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListType2CheckQuery {
    #[validate_extra(contains(pattern = &2))]
    pub list_type: Vec<u8>,
}
