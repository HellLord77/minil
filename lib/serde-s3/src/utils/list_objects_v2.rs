use serdev::Deserialize;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListObjectsV2CheckQuery {
    #[validate(custom(function = "validate_list_type"))]
    pub list_type: Vec<u8>,
}

fn validate_list_type(list_type: &[u8]) -> Result<(), ValidationError> {
    if list_type.contains(&2) {
        Ok(())
    } else {
        Err(ValidationError::new("list_type"))
    }
}
