use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::ChecksumType;
use crate::types::CompletedMultipartUpload;
use crate::types::RequestCharged;
use crate::types::RequestPayer;
use crate::types::ServerSideEncryption;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct CompleteMultipartUploadInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct CompleteMultipartUploadInputQuery {
    #[serde_rename_chain(convert_case = "camel")]
    pub upload_id: Uuid,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct CompleteMultipartUploadInputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub if_match: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub if_none_match: Option<String>,

    #[serde(rename = "x-amz-checksum-crc32")]
    pub checksum_crc32: Option<String>,

    #[serde(rename = "x-amz-checksum-crc32c")]
    pub checksum_crc32c: Option<String>,

    #[serde(rename = "x-amz-checksum-crc64nvme")]
    pub checksum_crc64nvme: Option<String>,

    #[serde(rename = "x-amz-checksum-sha1")]
    pub checksum_sha1: Option<String>,

    #[serde(rename = "x-amz-checksum-sha256")]
    pub checksum_sha256: Option<String>,

    pub checksum_type: Option<ChecksumType>,

    pub expected_bucket_owner: Option<String>,

    pub mp_object_size: Option<u64>,

    pub request_payer: Option<RequestPayer>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    pub server_side_encryption_customer_key: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,
}

pub type CompleteMultipartUploadInputBody = CompletedMultipartUpload;

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct CompleteMultipartUploadOutputHeader {
    pub expiration: Option<DateTime<Utc>>,

    pub request_charged: Option<RequestCharged>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub version_id: Option<Uuid>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "CompleteMultipartUploadResult", rename_all = "PascalCase")]
pub struct CompleteMultipartUploadOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub bucket: String,

    #[serde(rename = "ChecksumCRC32")]
    pub checksum_crc32: Option<String>,

    #[serde(rename = "ChecksumCRC32C")]
    pub checksum_crc32_c: Option<String>,

    #[serde(rename = "ChecksumCRC64NVME")]
    pub checksum_crc64_nvme: Option<String>,

    #[serde(rename = "ChecksumSHA1")]
    pub checksum_sha1: Option<String>,

    #[serde(rename = "ChecksumSHA256")]
    pub checksum_sha256: Option<String>,

    pub checksum_type: Option<ChecksumType>,

    pub e_tag: String,

    pub key: String,

    pub location: String,
}
