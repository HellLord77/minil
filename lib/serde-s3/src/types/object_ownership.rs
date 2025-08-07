use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum ObjectOwnership {
    BucketOwnerEnforced,
    BucketOwnerPreferred,
    ObjectWriter,
}
