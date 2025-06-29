use serdev::Deserialize;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct GetBucketVersioningCheckQuery {
    #[validate(custom(function = "validate_versioning"))]
    versioning: Vec<String>,
}

fn validate_versioning(versioning: &[String]) -> Result<(), ValidationError> {
    if versioning.contains(&"".to_owned()) {
        Ok(())
    } else {
        Err(ValidationError::new("versioning"))
    }
}
