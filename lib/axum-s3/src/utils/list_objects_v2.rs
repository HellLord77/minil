use axum_extra::extract::Query;
use axum_extra::extract::QueryRejection;
use serde_s3::utils::ListObjectsV2;

pub type ListObjectsV2Result = Result<Query<ListObjectsV2>, QueryRejection>;
