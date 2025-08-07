use bon::Builder;
use http_digest::DigestMd5;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::serde_as;
use serde_with_extra::DisplayFromBytes;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::ChecksumAlgorithm;
use crate::types::RequestCharged;
use crate::types::RequestPayer;
use crate::types::ServerSideEncryption;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct UploadPartInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
#[serde(validate = "Validate::validate")]
pub struct UploadPartInputQuery {
    #[validate(range(min = 1, max = 10_000))]
    #[serde_rename_chain(convert_case = "camel")]
    pub part_number: u16,

    #[serde_rename_chain(convert_case = "camel")]
    pub upload_id: Uuid,
}

#[serde_as]
#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
#[serde(validate = "Validate::validate")]
pub struct UploadPartInputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub content_length: Option<u64>,

    #[serde(rename = "Content-MD5")]
    #[serde_as(as = "Option<DisplayFromBytes>")]
    pub content_md5: Option<DigestMd5>,

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

    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,

    pub sdk_checksum_algorithm: Option<ChecksumAlgorithm>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    pub server_side_encryption_customer_key: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct UploadPartOutputHeader {
    #[serde_rename_chain(convert_case = "pascal")]
    pub e_tag: Option<String>,

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

    pub request_charged: Option<RequestCharged>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,
}
