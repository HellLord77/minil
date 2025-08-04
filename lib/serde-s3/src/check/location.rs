use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

#[validate_extra]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct LocationCheckQuery {
    #[validate_extra(contains(pattern = &"".to_owned()))]
    pub location: Vec<String>,
}
