pub mod create_bucket;
pub mod delete_bucket;
pub mod list_buckets;

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use aws_sdk_s3::operation::create_bucket::CreateBucketInput;
    use aws_sdk_s3::operation::create_bucket::CreateBucketOutput;
    use aws_sdk_s3::operation::delete_bucket::DeleteBucketInput;
    use aws_sdk_s3::operation::delete_bucket::DeleteBucketOutput;
    use aws_sdk_s3::operation::list_buckets::ListBucketsInput;
    use aws_sdk_s3::operation::list_buckets::ListBucketsOutput;
}
