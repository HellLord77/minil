use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use validator::Validate;

use crate::types::ArchiveStatus;
use crate::types::ChecksumMode;
use crate::types::ChecksumType;
use crate::types::ObjectLockLegalHoldStatus;
use crate::types::ObjectLockMode;
use crate::types::ReplicationStatus;
use crate::types::RequestCharged;
use crate::types::RequestPayer;
use crate::types::ServerSideEncryption;
use crate::types::StorageClass;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct HeadObjectInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_rename_chain]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct HeadObjectInputQuery {
    #[validate(range(min = 1, max = 10_000))]
    #[serde_rename_chain(convert_case = "camel")]
    pub part_number: Option<u16>,

    pub response_cache_control: Option<String>,

    pub response_content_disposition: Option<String>,

    pub response_content_encoding: Option<String>,

    pub response_content_language: Option<String>,

    pub response_content_type: Option<String>,

    pub response_expires: Option<DateTime<Utc>>,

    #[serde_rename_chain(convert_case = "camel")]
    pub version_id: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct HeadObjectInputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub if_match: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub if_modified_since: Option<DateTime<Utc>>,

    #[serde_rename_chain(convert_case = "train")]
    pub if_none_match: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub if_unmodified_since: Option<DateTime<Utc>>,

    #[serde_rename_chain(convert_case = "train")]
    pub range: Option<String>,

    pub checksum_mode: Option<ChecksumMode>,

    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    pub server_side_encryption_customer_key: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct HeadObjectOutputHeader {
    #[serde_rename_chain(convert_case = "kebab")]
    pub accept_ranges: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub cache_control: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_disposition: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_encoding: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_language: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_length: Option<u64>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_type: Option<String>,

    #[serde_rename_chain(convert_case = "pascal")]
    pub e_tag: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub expires: Option<DateTime<Utc>>,

    #[serde_rename_chain(convert_case = "train")]
    pub last_modified: Option<DateTime<Utc>>,

    pub archive_status: Option<ArchiveStatus>,

    pub checksum_crc32: Option<String>,

    pub checksum_crc32c: Option<String>,

    pub checksum_crc64nvme: Option<String>,

    pub checksum_sha1: Option<String>,

    pub checksum_sha256: Option<String>,

    pub checksum_type: Option<ChecksumType>,

    pub delete_marker: Option<bool>,

    pub expiration: Option<String>,

    pub missing_meta: Option<u16>,

    pub mp_parts_count: Option<u16>,

    pub object_lock_legal_hold: Option<ObjectLockLegalHoldStatus>,

    pub object_lock_mode: Option<ObjectLockMode>,

    pub object_lock_retain_until_date: Option<DateTime<Utc>>,

    pub replication_status: Option<ReplicationStatus>,

    pub request_charged: Option<RequestCharged>,

    pub restore: Option<String>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,

    pub storage_class: Option<StorageClass>,

    pub tagging_count: Option<u16>,

    pub version_id: Option<String>,

    pub website_redirect_location: Option<String>,
}
