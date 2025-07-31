use serde_rename_chain::serde_rename_chain;
use serdev::Deserialize;

#[serde_rename_chain(add_prefix = "x_amz_", convert_case = "train")]
#[derive(Debug, Deserialize)]
pub struct CommonExtInputHeader {
    pub id_2: String,

    pub request_id: String,
}
