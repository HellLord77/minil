use bon::Builder;
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use serde_with::skip_serializing_none;
use serdev::Deserialize;
use uuid::Uuid;
use validator::Validate;
use validator_extra::validate_extra;

use crate::types::ChecksumAlgorithm;
use crate::types::ChecksumType;
use crate::types::CommonPrefix;
use crate::types::Initiator;
use crate::types::Owner;
use crate::types::Part;
use crate::types::RequestCharged;
use crate::types::RequestPayer;
use crate::types::StorageClass;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListPartsInputPath {
    pub bucket: String,
}

#[validate_extra]
#[serde_inline_default]
#[serde_rename_chain(convert_case = "kebab")]
#[derive(Debug, Validate, Deserialize)]
#[serde(validate = "Validate::validate")]
pub struct ListPartsInputQuery {
    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_parts: u16,

    pub part_number_marker: Option<u16>,

    #[serde_rename_chain(convert_case = "camel")]
    pub upload_id: Uuid,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListPartsInputHeader {
    pub expected_bucket_owner: Option<String>,

    pub request_payer: Option<RequestPayer>,

    pub server_side_encryption_customer_algorithm: Option<String>,

    pub server_side_encryption_customer_key: Option<String>,

    #[serde(rename = "x-amz-server-side-encryption-customer-key-MD5")]
    pub server_side_encryption_customer_key_md5: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct ListPartsOutputHeader {
    pub abort_date: Option<DateTime<Utc>>,

    pub abort_rule_id: Option<Uuid>,

    pub request_charged: Option<RequestCharged>,
}

#[skip_serializing_none]
#[derive(Debug, Builder, Serialize)]
#[serde(rename = "ListPartsResult", rename_all = "PascalCase")]
pub struct ListPartsOutputBody {
    #[builder(default = "http://s3.amazonaws.com/doc/2006-03-01/")]
    #[serde(rename = "@xmlns")]
    pub xmlns: &'static str,

    pub bucket: Option<String>,

    pub checksum_algorithm: Option<ChecksumAlgorithm>,

    pub checksum_type: Option<ChecksumType>,

    pub common_prefixes: Vec<CommonPrefix>,

    pub initiator: Option<Initiator>,

    pub is_truncated: bool,

    pub key: String,

    pub max_parts: u16,

    pub next_part_number_marker: Option<String>,

    pub owner: Option<Owner>,

    pub part: Vec<Part>,

    pub part_number_marker: Option<u16>,

    pub storage_class: Option<StorageClass>,

    pub upload_id: Uuid,
}
