use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BucketLocationConstraint {
    AfSouth1,
    ApEast1,
    ApNorthEast1,
    ApNorthEast2,
    ApNorthEast3,
    ApSouth1,
    ApSouth2,
    ApSouthEast1,
    ApSouthEast2,
    ApSouthEast3,
    ApSouthEast4,
    ApSouthEast5,
    CaCentral1,
    CnNorth1,
    CnNorthWest1,
    #[serde(rename = "EU")]
    Eu,
    EuCentral1,
    EuCentral2,
    EuNorth1,
    EuSouth1,
    EuSouth2,
    EuWest1,
    EuWest2,
    EuWest3,
    IlCentral1,
    MeCentral1,
    MeSouth1,
    SaEast1,
    UsEast1,
    UsEast2,
    UsGovEast1,
    UsGovWest1,
    UsWest1,
    UsWest2,
}
