use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReplicationStatus {
    Complete,
    Completed,
    Failed,
    Pending,
    Replica,
}
