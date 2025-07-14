use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReplicationStatus {
    Complete,
    Completed,
    Failed,
    Pending,
    Replica,
}
