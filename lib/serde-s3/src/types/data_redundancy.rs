use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum DataRedundancy {
    SingleAvailabilityZone,
    SingleLocalZone,
}
