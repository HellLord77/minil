use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ObjectOwnership {
    BucketOwnerPreferred,
    ObjectWriter,
    BucketOwnerEnforced,
}
