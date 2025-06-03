pub mod operation;
pub mod types;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use aws_sdk_s3::operation::create_bucket::CreateBucketInput;
    use aws_sdk_s3::operation::create_bucket::CreateBucketOutput;
    use aws_sdk_s3::operation::delete_bucket::DeleteBucketInput;
    use aws_sdk_s3::operation::delete_bucket::DeleteBucketOutput;
    use aws_sdk_s3::operation::list_buckets::ListBucketsInput;
    use aws_sdk_s3::operation::list_buckets::ListBucketsOutput;
    use aws_sdk_s3::operation::list_objects::ListObjectsInput;
    use aws_sdk_s3::operation::list_objects::ListObjectsOutput;
    use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Input;
    use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
}
