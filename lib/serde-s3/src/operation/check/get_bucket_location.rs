use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_check;

#[validate_check]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct GetBucketLocationCheckQuery {
    #[validate_check(location.contains(&"".to_owned()))]
    pub location: Vec<String>,
}
