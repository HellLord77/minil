use bon::Builder;
use serde::Serialize;
use serde_inline_default::serde_inline_default;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use validator::Validate;
use validator_extra::validate_check;
use crate::types::EncodingType;
use crate::types::ListBucketResultV2;
use crate::types::OptionalObjectAttributes;
use crate::types::RequestPayer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListObjectsV2InputPath {
    pub bucket: String,
}

#[validate_check]
// #[serde_as]
#[serde_inline_default]
#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(validate = "Validate::validate")]
pub struct ListObjectsV2InputQuery {
    // #[serde_as(as = "NoneAsEmptyString")]
    pub continuation_token: Option<String>,

    // #[validator_check(equals(value = "/", ...))] => #[validator_check(eq(input = "\"/\"", ...))]
    // #[validator_check(eq(input = "\"/\"", ...))] => #[validator_check(check_ass_fn(ident = "eq", input = "\"/\"", ...))]
    // #[validator_check(check_ass_fn(ident = "eq", input = "\"/\"", ...))] => #[validator_check(check(check = "delimiter.eq(\"/\")", ...))]
    // unescape: \\ => \, \" => "
    // #[validator_check(check_fn(ident = "name", input = "\"arg\"", input = "delimiter", ...))] => #[validator_check(check(check = "name(\"arg\", delimiter)", ...))]
    // #[validator_check(check_ass_fn(ident = "name", input = "None", input = "var", ...))] => #[validator_check(check(check = "delimiter.name(None, var)", ...))]
    // #[validator_check(check(check = "delimiter == \"/\"", code = "delimiter", message = "..."))]
    #[validate_check(delimiter == "/")]
    // #[serde_as(as = "NoneAsEmptyString")]
    pub delimiter: Option<String>,

    pub encoding_type: Option<EncodingType>,

    pub fetch_owner: Option<bool>,

    #[validate(range(min = 1, max = 1_000))]
    #[serde_inline_default(1_000)]
    pub max_keys: u16,

    #[validate(length(min = 0, max = 1_024))]
    // #[serde_as(as = "NoneAsEmptyString")]
    pub prefix: Option<String>,

    // #[serde_as(as = "NoneAsEmptyString")]
    pub start_after: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Deserialize)]
pub struct ListObjectsV2InputHeader {
    pub expected_bucket_owner: Option<String>,

    pub optional_object_attributes: Option<Vec<OptionalObjectAttributes>>,

    pub request_payer: Option<RequestPayer>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "kebab")]
#[derive(Debug, Builder, Serialize)]
pub struct ListObjectsV2OutputHeader {
    pub request_charged: Option<RequestPayer>,
}

pub type ListObjectsV2OutputBody = ListBucketResultV2;
