use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LocationType {
    AvailabilityZone,
    LocalZone,
}
