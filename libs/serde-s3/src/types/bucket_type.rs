use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BucketType {
    Directory,
}
