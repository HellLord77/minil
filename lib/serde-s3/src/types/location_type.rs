use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum LocationType {
    AvailabilityZone,
    LocalZone,
}
