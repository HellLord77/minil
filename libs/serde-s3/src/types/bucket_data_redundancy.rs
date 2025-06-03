use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum BucketDataRedundancy {
    SingleAvailabilityZone,
    SingleLocalZone,
}
