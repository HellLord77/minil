use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use httpdate::HttpDate;
use mime::Mime;
use serde::Serialize;
use serde_rename_chain::serde_rename_chain;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with_extra::DisplayFromBytes;
use serde_with_extra::SerdeQuery;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::types::ChecksumAlgorithm;
use crate::types::ChecksumType;
use crate::types::ObjectCannedAcl;
use crate::types::ObjectLockLegalHoldStatus;
use crate::types::ObjectLockMode;
use crate::types::RequestCharged;
use crate::types::RequestPayer;
use crate::types::ServerSideEncryption;
use crate::types::StorageClass;
use crate::types::Tag;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct CreateMultipartUploadInputPath {
    pub bucket: String,

    #[validate(length(min = 1))]
    pub key: String,
}

#[serde_as]
#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct CreateMultipartUploadInputHeader {
    #[serde_rename_chain(convert_case = "train")]
    pub cache_control: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_disposition: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_encoding: Option<String>,

    #[serde_rename_chain(convert_case = "train")]
    pub content_language: Option<String>,

    #[serde_as(as = "Option<DisplayFromBytes>")]
    #[serde_rename_chain(convert_case = "train")]
    pub content_type: Option<Mime>,

    #[serde_as(as = "Option<DisplayFromBytes>")]
    #[serde_rename_chain(convert_case = "train")]
    pub expires: Option<HttpDate>,

    pub acl: Option<ObjectCannedAcl>,

    pub checksum_algorithm: Option<ChecksumAlgorithm>,

    pub checksum_type: Option<ChecksumType>,

    pub expected_bucket_owner: Option<String>,

    pub grant_full_control: Option<String>,

    pub grant_read: Option<String>,

    pub grant_read_acp: Option<String>,

    pub grant_write_acp: Option<String>,

    pub object_lock_legal_hold: Option<ObjectLockLegalHoldStatus>,

    pub object_lock_mode: Option<ObjectLockMode>,

    pub object_lock_retain_until_date: Option<DateTime<Utc>>,

    pub request_payer: Option<RequestPayer>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub server_side_encryption_context: Option<String>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    pub server_side_encryption_customer_key: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,

    pub storage_class: Option<StorageClass>,

    #[serde_as(as = "Option<SerdeQuery>")]
    pub tagging: Option<Vec<Tag>>,

    pub website_redirect_location: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct CreateMultipartUploadOutputHeader {
    pub abort_date: Option<DateTime<Utc>>,

    pub abort_rule_id: Option<Uuid>,

    pub checksum_algorithm: Option<ChecksumAlgorithm>,

    pub checksum_type: Option<ChecksumType>,

    pub request_charged: Option<RequestCharged>,

    pub server_side_encryption: Option<ServerSideEncryption>,

    pub server_side_encryption_aws_kms_key_id: Option<String>,

    pub server_side_encryption_bucket_key_enabled: Option<bool>,

    pub server_side_encryption_encryption_context: Option<String>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "InitiateMultipartUploadResult", rename_all = "PascalCase")]
pub struct CreateMultipartUploadOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub bucket: String,

    pub key: String,

    pub upload_id: Uuid,
}
