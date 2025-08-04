use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_extra;

#[validate_extra]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct VersionsCheckQuery {
    #[validate_extra(contains(pattern = &"".to_owned()))]
    pub versions: Vec<String>,
}
