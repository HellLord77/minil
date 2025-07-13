use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerSideEncryption {
    #[serde(rename = "AES256")]
    Aes256,
    #[serde(rename = "aws:fsx")]
    AwsFsx,
    #[serde(rename = "aws:kms")]
    AwsKms,
    #[serde(rename = "aws:kms:dsse")]
    AwsKmsDsse,
}
