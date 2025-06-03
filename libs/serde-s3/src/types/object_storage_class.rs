use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectStorageClass {
    Standard,
    ReducedRedundancy,
    Glacier,
    StandardIa,
    OnezoneIa,
    IntelligentTiering,
    DeepArchive,
    Outposts,
    GlacierIr,
    Snow,
    ExpressOnezone,
}
