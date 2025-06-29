use axum_extra::extract::Query;
use axum_extra::extract::QueryRejection;
use serde_s3::utils::GetBucketVersioning;

pub type GetBucketVersioningResult = Result<Query<GetBucketVersioning>, QueryRejection>;
