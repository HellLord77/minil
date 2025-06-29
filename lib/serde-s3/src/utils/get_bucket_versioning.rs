use serdev::Deserialize;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct GetBucketVersioning {
    #[validate(custom(function = "validate_versioning"))]
    pub versioning: Vec<String>,
}

fn validate_versioning(versioning: &[String]) -> Result<(), ValidationError> {
    if versioning.contains(&"".to_owned()) {
        Ok(())
    } else {
        Err(ValidationError::new("versioning"))
    }
}
