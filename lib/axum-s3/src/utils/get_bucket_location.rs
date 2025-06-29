use axum_extra::extract::Query;
use axum_extra::extract::QueryRejection;
use serde_s3::utils::GetBucketLocation;

pub type GetBucketLocationResult = Result<Query<GetBucketLocation>, QueryRejection>;
