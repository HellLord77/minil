use serdev::Deserialize;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct GetBucketLocationCheckQuery {
    #[validate(custom(function = "validate_location"))]
    pub location: Vec<String>,
}

fn validate_location(location: &[String]) -> Result<(), ValidationError> {
    if location.contains(&"".to_owned()) {
        Ok(())
    } else {
        Err(ValidationError::new("location"))
    }
}
