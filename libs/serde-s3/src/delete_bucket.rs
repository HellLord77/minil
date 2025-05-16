use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeleteBucketHeader {
    pub x_amz_expected_bucket_owner: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteBucketQuery {
    pub bucket: String,
}
