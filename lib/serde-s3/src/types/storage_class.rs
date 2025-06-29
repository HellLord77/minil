use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StorageClass {
    DeepArchive,
    ExpressOnezone,
    Glacier,
    GlacierIr,
    IntelligentTiering,
    OnezoneIa,
    Outposts,
    ReducedRedundancy,
    Snow,
    Standard,
    StandardIa,
}
