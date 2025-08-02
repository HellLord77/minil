use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;
use validator::Validate;

use crate::utils::Algorithm;

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(validate = "Validate::validate")]
pub struct CommonInputPath {
    pub bucket: Option<String>,

    #[validate(length(min = 1))]
    pub key: Option<String>,
}

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "train")]
#[derive(Debug, Deserialize)]
pub struct CommonInputHeader {
    #[serde_rename_chain(convert_case = "pascal")]
    pub action: Option<String>,

    #[serde_rename_chain(convert_case = "pascal")]
    pub version: Option<NaiveDate>,

    pub algorithm: Option<Algorithm>,

    pub credential: Option<String>,

    pub date: Option<DateTime<Utc>>,

    pub security_token: Option<String>,

    pub signature: Option<String>,

    #[serde(rename = "X-Amz-SignedHeaders")]
    pub signed_headers: Option<String>,
}
