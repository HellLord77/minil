use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use httpdate::HttpDate;
use mime::Mime;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use serde_with_extra::AsString;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::ChecksumAlgorithm;
use crate::types::ChecksumType;
use crate::types::ObjectCannedAcl;
use crate::types::ObjectLockLegalHoldStatus;
use crate::types::ObjectLockMode;
use crate::types::RequestPayer;
use crate::types::ServerSideEncryption;
use crate::types::StorageClass;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct PutObjectInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_as]
#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
#[serde(validate = "Validate::validate")]
pub struct PutObjectInputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub cache_control: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_disposition: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_encoding: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_language: Option<String>,

    #[serde(rename = "Content-MD5")]
    pub content_md5: Option<String>,

    #[serde_as(as = "Option<AsString<DisplayFromStr>>")]
    #[serde_rename_chain(convert_case = "train")]
    pub content_type: Option<Mime>,

    #[serde_as(as = "Option<AsString<DisplayFromStr>>")]
    #[serde_rename_chain(convert_case = "train")]
    pub expires: Option<HttpDate>,

    #[serde_rename_chain(convert_case = "train")]
    pub if_match: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub if_none_match: Option<String>,

    pub acl: Option<ObjectCannedAcl>,

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

    pub grant_full_control: Option<String>,

    pub grant_read: Option<String>,

    pub grant_read_acp: Option<String>,

    pub grant_write_acp: Option<String>,

    pub object_lock_legal_hold: Option<ObjectLockLegalHoldStatus>,

    pub object_lock_mode: Option<ObjectLockMode>,

    pub object_lock_retain_until_date: Option<DateTime<Utc>>,

    pub request_payer: Option<RequestPayer>,

    pub sdk_checksum_algorithm: Option<ChecksumAlgorithm>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub server_side_encryption_context: Option<String>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    pub server_side_encryption_customer_key: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,

    pub storage_class: Option<StorageClass>,

    pub tagging: Option<String>,

    pub website_redirect_location: Option<String>,

    pub write_offset_bytes: Option<u64>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct PutObjectOutputHeader {
    #[serde_rename_chain(convert_case = "pascal")]
    pub e_tag: String,

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

    pub expiration: Option<String>,

    pub object_size: Option<u64>,

    pub request_charged: Option<RequestPayer>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub server_side_encryption_encryption_context: Option<String>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,

    pub version_id: Option<Uuid>,
}
